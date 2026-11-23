//! Stadtsimulation in Echtzeit: Lager und Kapazität, Produktionsketten, Arbeitskräfte,
//! Bevölkerung (Bedürfnisse, Zufriedenheit, Steuern, Wohnstufen), Markt und Ausbaustufen.

use crate::chronicle::ChronicleKind;
use crate::data::{BuildingDef, GameData, Item, ResourceKind, ServiceKind};
use crate::entity::*;
use crate::events::SimEvent;
use crate::world::SimWorld;
use glam::{IVec2, Vec2};

// ---------------------------------------------------------------------------------------------
// Lager, Kosten und Preise
// ---------------------------------------------------------------------------------------------

impl SimWorld {
    /// Kapazität eines Items für `owner` (Gold und Nicht-Spieler sind unbegrenzt).
    pub fn capacity_of(&self, owner: PlayerId, it: Item) -> f32 {
        if owner != PLAYER || it == Item::Res(ResourceKind::Gold) { f32::INFINITY } else { self.players[owner as usize].capacity }
    }

    /// Schreibt `amount` gut; überschüssige Menge über der Lagerkapazität verfällt. Gibt die
    /// tatsächlich aufgenommene Menge zurück.
    pub fn credit(&mut self, owner: PlayerId, it: Item, amount: f32) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }
        let cap = self.capacity_of(owner, it);
        let p = &mut self.players[owner as usize];
        let cur = p.stock(it);
        let accepted = amount.min((cap - cur).max(0.0));
        *p.stock_mut(it) += accepted;
        accepted
    }

    /// Entnimmt `amount`, wenn genug vorhanden ist.
    pub fn take(&mut self, owner: PlayerId, it: Item, amount: f32) -> bool {
        let p = &mut self.players[owner as usize];
        if p.stock(it) + 1e-3 < amount {
            return false;
        }
        let s = p.stock_mut(it);
        *s = (*s - amount).max(0.0);
        true
    }

    /// Prüft Grundkosten und Waren; liefert das erste fehlende Item.
    pub fn afford(&self, owner: PlayerId, cost: &crate::data::Cost, goods: &[(Item, f32)]) -> Result<(), Item> {
        let p = &self.players[owner as usize];
        if let Some(k) = p.resources.first_missing(cost) {
            return Err(Item::Res(k));
        }
        for (it, n) in goods {
            if p.stock(*it) + 1e-3 < *n {
                return Err(*it);
            }
        }
        Ok(())
    }

    /// Zieht Kosten ab (nach erfolgreichem [`afford`](Self::afford)).
    pub fn pay(&mut self, owner: PlayerId, cost: &crate::data::Cost, goods: &[(Item, f32)]) {
        let p = &mut self.players[owner as usize];
        p.resources.sub(cost);
        for (it, n) in goods {
            let s = p.stock_mut(*it);
            *s = (*s - n).max(0.0);
        }
    }

    /// Erstattet einen Teil der Kosten (ohne Kapazitätsgrenze, damit nichts verloren geht).
    pub fn refund(&mut self, owner: PlayerId, cost: &crate::data::Cost, goods: &[(Item, f32)], factor: f32) {
        let p = &mut self.players[owner as usize];
        p.resources.add(&cost.scaled(factor));
        for (it, n) in goods {
            *p.stock_mut(*it) += n * factor;
        }
    }

    /// Aktueller Marktpreis (ohne Handelsspanne).
    pub fn market_price(&self, data: &GameData, owner: PlayerId, it: Item) -> f32 {
        let c = &data.rules.city;
        let pressure = self.players[owner as usize].market.get(it.index()).copied().unwrap_or(0.0);
        data.base_price(it) * (1.0 + pressure).clamp(c.market_min, c.market_max)
    }

    /// Mittelpunkt eines fertigen Marktes von `owner`.
    pub fn market_of(&self, data: &GameData, owner: PlayerId) -> Option<Vec2> {
        self.buildings.iter().find(|(_, b)| b.owner == owner && b.complete && data.building(b.kind).market).map(|(_, b)| b.center())
    }
}

/// Fehlermeldung für ein fehlendes Item.
pub fn missing_text(data: &GameData, it: Item) -> String {
    format!("Nicht genug {}.", data.item_name(it))
}

// ---------------------------------------------------------------------------------------------
// Standort
// ---------------------------------------------------------------------------------------------

fn footprint_center(def: &BuildingDef, tile: IVec2) -> Vec2 {
    Vec2::new(tile.x as f32 + def.size.0 as f32 * 0.5, tile.y as f32 + def.size.1 as f32 * 0.5)
}

/// Prüft die Standortbedingung eines Gewinnungsgebäudes (Bäume, Erzader, Gelände).
pub fn check_site(world: &SimWorld, _data: &GameData, def: &BuildingDef, tile: IVec2) -> Result<(), String> {
    let Some(site) = &def.site else { return Ok(()) };
    let hint = if site.hint.is_empty() { "Hier fehlen die nötigen Vorkommen.".to_string() } else { site.hint.clone() };
    if let Some(nk) = site.node {
        let c = footprint_center(def, tile);
        let n = world.nodes.iter().filter(|(_, nd)| nd.kind == nk && nd.amount > 0 && nd.distance_to(c) <= site.node_radius).count() as u32;
        if n < site.node_min.max(1) {
            return Err(hint);
        }
    }
    if !site.terrain.is_empty() {
        let r = site.terrain_radius.max(1);
        let mut n = 0;
        for y in tile.y - r..tile.y + def.size.1 as i32 + r {
            for x in tile.x - r..tile.x + def.size.0 as i32 + r {
                if site.terrain.contains(&world.map.terrain_at(IVec2::new(x, y))) {
                    n += 1;
                }
            }
        }
        if n < site.terrain_min {
            return Err(hint);
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Kapazität
// ---------------------------------------------------------------------------------------------

/// Lagerkapazität des Spielers aus fertigen Lagergebäuden und Ausbaustufen neu berechnen.
pub fn recompute_capacity(world: &mut SimWorld, data: &GameData) {
    let mut cap: f32 = world.buildings.iter().filter(|(_, b)| b.owner == PLAYER && b.complete).map(|(_, b)| data.building(b.kind).storage).sum();
    let era = world.players[PLAYER as usize].era as usize;
    cap += data.rules.eras.iter().take(era).map(|e| e.storage_bonus).sum::<f32>();
    world.players[PLAYER as usize].capacity = cap + world.players[PLAYER as usize].extra_capacity;
}

// ---------------------------------------------------------------------------------------------
// Takt
// ---------------------------------------------------------------------------------------------

pub fn tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    production_tick(world, data, dt);
    era_tick(world, data, dt);
    if world.tick.is_multiple_of(data.rules.tick_rate as u64) {
        per_second(world, data);
    }
}

fn per_second(world: &mut SimWorld, data: &GameData) {
    recompute_capacity(world, data);
    allocate_workers(world, data);
    housing_second(world, data);
    // Handelsdruck klingt ab.
    let rec = data.rules.city.market_recovery;
    for p in &mut world.players {
        for m in &mut p.market {
            *m *= 1.0 - rec;
            if m.abs() < 1e-3 {
                *m = 0.0;
            }
        }
        p.unrest_cooldown = (p.unrest_cooldown - 1.0).max(0.0);
        p.full_cooldown = (p.full_cooldown - 1.0).max(0.0);
    }
}

// ---------------------------------------------------------------------------------------------
// Arbeitskräfte
// ---------------------------------------------------------------------------------------------

fn allocate_workers(world: &mut SimWorld, data: &GameData) {
    let ratio = data.rules.city.workforce_ratio;
    let residents: f32 = world.buildings.iter().filter(|(_, b)| b.owner == PLAYER && b.complete && data.building(b.kind).housing.is_some()).map(|(_, b)| b.residents).sum();
    let workforce = (residents * ratio).floor() as u32;
    let mut remaining = workforce;
    let mut used = 0;
    let ids = world.buildings.ids();
    for id in ids {
        let Some(b) = world.buildings.get_mut(id) else { continue };
        if b.owner != PLAYER {
            continue;
        }
        let def = data.building(b.kind);
        if !b.complete || !def.employs() {
            b.workers = 0;
            continue;
        }
        let want = b.worker_limit.unwrap_or(def.workers).min(def.workers);
        let give = want.min(remaining);
        b.workers = give;
        remaining -= give;
        used += give;
    }
    let p = &mut world.players[PLAYER as usize];
    p.residents = residents;
    p.workforce = workforce;
    p.workers_used = used;
}

// ---------------------------------------------------------------------------------------------
// Produktion
// ---------------------------------------------------------------------------------------------

/// Nächste Quelle `kind` mit Vorrat im Radius.
fn nearest_node(world: &SimWorld, kind: NodeKind, c: Vec2, radius: f32) -> Option<NodeId> {
    world.nodes.iter().filter(|(_, n)| n.kind == kind && n.amount > 0).map(|(id, n)| (id, n.distance_to(c))).filter(|(_, d)| *d <= radius).min_by(|a, b| a.1.total_cmp(&b.1)).map(|(id, _)| id)
}

fn production_tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    let radius = data.rules.city.logistics_radius;
    let depots: Vec<(PlayerId, Vec2)> = world.buildings.iter().filter(|(_, b)| b.complete && data.building(b.kind).storage > 0.0).map(|(_, b)| (b.owner, b.center())).collect();
    let ids = world.buildings.ids();
    for id in ids {
        let Some(b) = world.buildings.get(id) else { continue };
        let def = data.building(b.kind);
        if !b.complete || def.recipes.is_empty() {
            continue;
        }
        let (owner, center, workers, active, limit) = (b.owner, b.center(), b.workers, b.cycle_active, b.worker_limit);
        let recipe_idx = (b.recipe as usize).min(def.recipes.len() - 1);
        let recipe = &def.recipes[recipe_idx];
        let set = |world: &mut SimWorld, status: ProdStatus, stall: Option<Item>| {
            if let Some(b) = world.buildings.get_mut(id) {
                b.status = status;
                b.stall_item = stall;
            }
        };
        if limit == Some(0) {
            set(world, ProdStatus::Disabled, None);
            continue;
        }
        if !depots.iter().any(|(o, p)| *o == owner && (*p - center).length() <= radius) {
            set(world, ProdStatus::NoDepot, None);
            continue;
        }
        if workers == 0 {
            set(world, ProdStatus::NoWorkers, None);
            continue;
        }
        if !active {
            // Zyklusbeginn: Standort, Eingaben und Platz im Lager prüfen.
            let mut node: Option<(NodeId, f32)> = None;
            if let Some(site) = &def.site {
                let c = center;
                if let Some(nk) = site.node {
                    let found = nearest_node(world, nk, c, site.node_radius);
                    match found {
                        None => {
                            set(world, ProdStatus::NoSite, None);
                            continue;
                        }
                        Some(n) if site.consume_node => node = Some((n, recipe.outputs_r.first().map(|o| o.1).unwrap_or(1.0))),
                        Some(_) => {}
                    }
                }
            }
            if let Some((it, _)) = recipe.inputs_r.iter().find(|(it, n)| world.players[owner as usize].stock(*it) + 1e-3 < *n) {
                set(world, ProdStatus::MissingInput, Some(*it));
                continue;
            }
            if let Some((it, _)) = recipe.outputs_r.iter().find(|(it, _)| {
                let cap = world.capacity_of(owner, *it);
                world.players[owner as usize].stock(*it) >= cap
            }) {
                set(world, ProdStatus::StorageFull, Some(*it));
                continue;
            }
            for (it, n) in &recipe.inputs_r {
                world.take(owner, *it, *n);
            }
            if let Some((nid, amount)) = node {
                take_from_node(world, nid, amount);
            }
            if let Some(b) = world.buildings.get_mut(id) {
                b.cycle_active = true;
                b.cycle = 0.0;
            }
        }
        let eff = (workers as f32 / def.workers.max(1) as f32).min(1.0);
        let Some(b) = world.buildings.get_mut(id) else { continue };
        b.cycle += dt * eff;
        b.status = ProdStatus::Running;
        b.stall_item = None;
        if b.cycle >= recipe.cycle {
            b.cycle = 0.0;
            b.cycle_active = false;
            let kind = b.kind;
            for (it, n) in &recipe.outputs_r {
                world.credit(owner, *it, *n);
                if owner == PLAYER {
                    let p = &mut world.players[owner as usize];
                    match it {
                        Item::Res(k) => *p.stats.gathered.get_mut(*k) += *n,
                        Item::Good(_) => {}
                    }
                }
            }
            world.events.push(SimEvent::ProductionCycle { building: id, kind, owner, pos: center });
        }
    }
}

fn take_from_node(world: &mut SimWorld, nid: NodeId, amount: f32) {
    let Some(n) = world.nodes.get_mut(nid) else { return };
    let take = (amount.ceil() as u32).min(n.amount);
    n.amount -= take;
    if n.amount == 0 {
        let (tile, kind, center) = (n.tile, n.kind, n.center());
        world.nodes.remove(nid);
        world.map.occupy(tile, kind.size(), false);
        world.events.push(SimEvent::NodeDepleted { pos: center, tree: kind == NodeKind::Tree });
    }
}

// ---------------------------------------------------------------------------------------------
// Bevölkerung
// ---------------------------------------------------------------------------------------------

/// Wirkbereiche von Diensten (Tempel, Wehranlagen) eines Spielers.
fn service_centers(world: &SimWorld, data: &GameData, owner: PlayerId) -> Vec<(ServiceKind, Vec2, f32)> {
    world
        .buildings
        .iter()
        .filter(|(_, b)| b.owner == owner && b.complete)
        .filter_map(|(_, b)| {
            let def = data.building(b.kind);
            let s = def.service?;
            // Dienste mit Rezept (Tempel) wirken nur, wenn sie arbeiten.
            if !def.recipes.is_empty() && b.status != ProdStatus::Running {
                return None;
            }
            Some((s.kind, b.center(), s.radius))
        })
        .collect()
}

fn covered(centers: &[(ServiceKind, Vec2, f32)], kind: ServiceKind, p: Vec2) -> bool {
    centers.iter().any(|(k, c, r)| *k == kind && (*c - p).length() <= *r)
}

fn housing_second(world: &mut SimWorld, data: &GameData) {
    let rules = &data.rules.city;
    let centers = service_centers(world, data, PLAYER);
    let level = &rules.tax_levels[(world.players[PLAYER as usize].tax_level as usize).min(rules.tax_levels.len() - 1)];
    let era = world.players[PLAYER as usize].era;
    let mut tax_total = 0.0;
    let mut sum_h = 0.0;
    let mut sum_r = 0.0;
    let mut unrest = false;
    let mut changed = false;
    let ids = world.buildings.ids();
    for id in ids {
        let Some(b) = world.buildings.get(id) else { continue };
        let def = data.building(b.kind);
        let Some(h) = &def.housing else { continue };
        if !b.complete || b.owner != PLAYER {
            continue;
        }
        let (pos, r0, hap0) = (b.center(), b.residents, b.happiness);
        let cap = h.capacity as f32;
        // Bedürfnisse prüfen und Verbrauch abbuchen.
        let (mut sat, mut total, mut unmet) = (0.0, 0.0, 0u32);
        for (i, need) in h.needs.iter().enumerate() {
            total += need.weight;
            let met = {
                match &need.kind {
                    crate::data::NeedKind::Item(_) => match need.item_r {
                        Some(it) => {
                            let demand = r0 * need.rate / 60.0;
                            if r0 < 0.5 {
                                // Leeres Haus: kein Verbrauch, aber ohne Vorrat zieht niemand ein.
                                f32::from(world.players[PLAYER as usize].stock(it) >= 1.0)
                            } else if demand <= 1e-6 {
                                1.0
                            } else {
                                let stock = world.players[PLAYER as usize].stock(it);
                                let got = stock.min(demand);
                                *world.players[PLAYER as usize].stock_mut(it) -= got;
                                got / demand
                            }
                        }
                        None => 1.0,
                    },
                    crate::data::NeedKind::Faith => f32::from(covered(&centers, ServiceKind::Faith, pos)),
                    crate::data::NeedKind::Safety => f32::from(covered(&centers, ServiceKind::Safety, pos)),
                }
            };
            sat += need.weight * met;
            if met < 0.95 {
                unmet |= 1 << i;
            }
        }
        let satisfaction = if total > 0.0 { sat / total } else { 1.0 };
        let target = (satisfaction * 100.0 + level.happiness).clamp(0.0, 100.0);
        let hap = hap0 + (target - hap0).clamp(-rules.happiness_rate, rules.happiness_rate);
        // Zu- und Abwanderung.
        let mut r = r0;
        if r > cap {
            r = (r - 0.5).max(cap);
        } else if hap < rules.unrest_below {
            r = (r - rules.emigration_rate).max(0.0);
        } else if hap >= rules.growth_min_happiness && r < cap {
            r = (r + rules.growth_rate).min(cap);
        }
        // Steuern.
        if hap >= rules.unrest_below {
            tax_total += r * h.tax * level.income * (hap / 60.0).clamp(0.0, 1.0);
        } else if r > 1.0 {
            unrest = true;
        }
        sum_h += hap * r;
        sum_r += r;
        // Aufstieg und Abstieg.
        let (mut up_t, mut down_t) = {
            let b = world.buildings.get(id).expect("exists");
            (b.up_timer, b.down_timer)
        };
        if hap >= rules.upgrade_above { up_t += 1.0 } else { up_t = 0.0 }
        if hap < rules.unrest_below { down_t += 1.0 } else { down_t = 0.0 }
        let mut new_kind = None;
        if up_t >= rules.upgrade_time
            && let Some(next) = h.upgrade_to.as_deref().and_then(|n| data.building_id(n))
            && era >= h.upgrade_era
            && next_tier_ready(world, data, next, pos, &centers)
        {
            new_kind = Some((next, true));
        } else if down_t >= rules.downgrade_time
            && let Some(prev) = h.downgrade_to.as_deref().and_then(|n| data.building_id(n))
        {
            new_kind = Some((prev, false));
        }
        let Some(b) = world.buildings.get_mut(id) else { continue };
        b.residents = r;
        b.happiness = hap;
        b.unmet = unmet;
        b.up_timer = up_t;
        b.down_timer = down_t;
        if let Some((nk, up)) = new_kind {
            let nd = data.building(nk);
            let frac = (b.hp / b.max_hp).clamp(0.0, 1.0);
            b.kind = nk;
            b.max_hp = nd.hp;
            b.hp = nd.hp * frac;
            b.up_timer = 0.0;
            b.down_timer = 0.0;
            if !up {
                b.happiness = 30.0;
            }
            let (center, kind) = (b.center(), b.kind);
            world.events.push(SimEvent::HouseChanged { id, kind, owner: PLAYER, pos: center, up });
            changed = true;
        }
    }
    let p = &mut world.players[PLAYER as usize];
    p.resources.gold += tax_total;
    p.stats.gathered.gold += tax_total;
    p.tax_rate = tax_total;
    p.avg_happiness = if sum_r > 0.0 { sum_h / sum_r } else { 0.0 };
    p.unrest = unrest;
    if unrest && p.unrest_cooldown <= 0.0 {
        p.unrest_cooldown = 120.0;
        world.events.push(SimEvent::Unrest { owner: PLAYER });
        world.record(ChronicleKind::Omen, "Unruhe ging durch Brennhollow: Die Bürger murrten über Steuern und Not.");
    }
    if changed {
        crate::systems::economy::recompute_population(world, data);
    }
}

/// Sind die Bedürfnisse der nächsten Wohnstufe sofort erfüllbar?
fn next_tier_ready(world: &SimWorld, data: &GameData, next: crate::data::BuildingTypeId, pos: Vec2, centers: &[(ServiceKind, Vec2, f32)]) -> bool {
    let Some(h) = &data.building(next).housing else { return true };
    h.needs.iter().all(|n| match &n.kind {
        crate::data::NeedKind::Item(_) => n.item_r.is_none_or(|it| world.players[PLAYER as usize].stock(it) >= 1.0),
        crate::data::NeedKind::Faith => covered(centers, ServiceKind::Faith, pos),
        crate::data::NeedKind::Safety => covered(centers, ServiceKind::Safety, pos),
    })
}

// ---------------------------------------------------------------------------------------------
// Markt
// ---------------------------------------------------------------------------------------------

/// Kauf (`amount > 0`) oder Verkauf (`amount < 0`) am Markt. Jede gehandelte Einheit verschiebt
/// den Preis; Kauf ist um die Handelsspanne teurer, Verkauf billiger als der Marktpreis.
pub fn trade(world: &mut SimWorld, data: &GameData, owner: PlayerId, item: Item, amount: i32) {
    if item == Item::Res(ResourceKind::Gold) || amount == 0 {
        return;
    }
    let Some(pos) = world.market_of(data, owner) else {
        world.error(owner, "Ihr braucht einen Markt, um zu handeln.");
        return;
    };
    let rules = data.rules.city.clone();
    let idx = item.index();
    let mut done = 0;
    let mut gold = 0.0;
    for _ in 0..amount.abs() {
        let price = world.market_price(data, owner, item);
        if amount > 0 {
            let cost = price * (1.0 + rules.market_fee);
            if world.players[owner as usize].resources.gold + 1e-3 < cost {
                world.error(owner, "Nicht genug Gold.");
                break;
            }
            if world.credit(owner, item, 1.0) < 1.0 {
                world.error(owner, "Lager voll – baut ein Lagerhaus.");
                break;
            }
            world.players[owner as usize].resources.gold -= cost;
            let m = &mut world.players[owner as usize].market[idx];
            *m = (*m + rules.market_impact).min(3.0);
            gold += cost;
            done += 1;
        } else {
            if !world.take(owner, item, 1.0) {
                world.error(owner, format!("Nicht genug {} zum Verkaufen.", data.item_name(item)));
                break;
            }
            let income = price * (1.0 - rules.market_fee);
            world.players[owner as usize].resources.gold += income;
            let m = &mut world.players[owner as usize].market[idx];
            *m = (*m - rules.market_impact).max(-0.8);
            gold += income;
            done -= 1;
        }
    }
    if done != 0 {
        world.events.push(SimEvent::Traded { owner, item, amount: done, gold, pos });
    }
}

// ---------------------------------------------------------------------------------------------
// Ausbaustufen
// ---------------------------------------------------------------------------------------------

/// Beginnt den Ausbau zur nächsten Stufe (Bergfried → Burg).
pub fn advance_era(world: &mut SimWorld, data: &GameData, owner: PlayerId) {
    let p = &world.players[owner as usize];
    let Some(next) = data.rules.eras.get(p.era as usize) else {
        world.error(owner, "Die höchste Ausbaustufe ist bereits erreicht.");
        return;
    };
    if p.era_progress.is_some() {
        world.error(owner, "Der Ausbau läuft bereits.");
        return;
    }
    for (bid, n) in &next.requires {
        let Some(bt) = data.building_id(bid) else { continue };
        let have = world.buildings.iter().filter(|(_, b)| b.owner == owner && b.kind == bt && b.complete).count() as u32;
        if have < *n {
            world.error(owner, format!("Benötigt: {n}× {} (fertig: {have})", data.building(bt).name));
            return;
        }
    }
    if let Err(it) = world.afford(owner, &next.cost, &next.goods_r) {
        world.error(owner, missing_text(data, it));
        return;
    }
    let (cost, goods) = (next.cost, next.goods_r.clone());
    world.pay(owner, &cost, &goods);
    world.players[owner as usize].era_progress = Some(0.0);
}

fn era_tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    let p = &mut world.players[PLAYER as usize];
    let Some(progress) = p.era_progress else { return };
    let Some(next) = data.rules.eras.get(p.era as usize) else {
        p.era_progress = None;
        return;
    };
    let progress = progress + dt / next.build_time.max(1.0);
    if progress < 1.0 {
        p.era_progress = Some(progress);
        return;
    }
    p.era_progress = None;
    p.era += 1;
    let (era, name) = (p.era, next.name.clone());
    world.events.push(SimEvent::EraAdvanced { owner: PLAYER, era });
    world.record(ChronicleKind::Construction, format!("Brennhollow wurde zur {name} ausgebaut."));
    recompute_capacity(world, data);
    crate::systems::economy::recompute_population(world, data);
}

/// Setzt das Tor eines Gebäudes (offen = für eigene Einheiten passierbar).
pub fn set_gate(world: &mut SimWorld, data: &GameData, id: BuildingId, open: bool) {
    let Some(b) = world.buildings.get_mut(id) else { return };
    if !b.complete || data.building(b.kind).fort != Some(crate::data::Fort::Gate) || b.gate_open == open {
        return;
    }
    b.gate_open = open;
    let (tile, owner, pos) = (b.tile, b.owner, b.center());
    world.map.set_gate(tile, owner, open);
    world.events.push(SimEvent::GateToggled { id, pos, open });
}
