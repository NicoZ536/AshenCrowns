//! Taktische Kampf-KI für reine Kampfmodi (Weltkarten-Schlachten, Gefechte ohne Wirtschaft).
//!
//! Die Funktion [`drive`] führt eine Seite mit denselben Befehlsmitteln wie der Spieler (Aufträge an Einheiten),
//! ist zustandslos und deterministisch (keine Zufallszahlen, feste Iterationsreihenfolge) und wird alle ein bis
//! zwei Sekunden aufgerufen. Sie ergänzt das Standardverhalten der Einheiten (Selbstverteidigung, Zielsuche) um:
//!
//! * **Fokusfeuer**: Jede Einheit wählt unter den Gegnern in Reichweite das Ziel mit dem besten Wert
//!   (gefährliche Klassen, angeschlagene Ziele, kurze Wege, Ziele, die Kameraden bereits angreifen).
//! * **Fernkämpfer hinter dem Nahkampf**: Schützen und Heiler halten Abstand hinter der eigenen Front und
//!   weichen Nahkämpfern aus, solange eigene Nahkämpfer sie decken.
//! * **Rückzug Angeschlagener**: Einheiten unter 30 % Leben ziehen sich hinter die Linien zurück
//!   (solange genügend gesunde Kameraden kämpfen) und halten dort die Stellung.
//! * **Belagerungswaffen** brechen zuerst Tore, dann Türme und Mauern, danach die übrigen Gebäude.
//! * **Verteidiger einer Wehranlage** bemannen Türme und Mauerabschnitte auf der Seite des Feindes
//!   (Fernkämpfer), halten die Nahkämpfer am Einfallstor und werfen sich nach einem Durchbruch in den Nahkampf.

use crate::data::{Fort, GameData, UnitClass};
use crate::entity::*;
use crate::systems::commands;
use crate::world::SimWorld;
use glam::Vec2;

/// Rolle der geführten Seite in der Schlacht.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Attacker,
    Defender,
}

/// Unter diesem Lebensanteil ziehen sich Einheiten zurück.
const WOUNDED: f32 = 0.3;
/// Zusätzliche Wahrnehmungsreichweite für Fokusziele (Kacheln über die Waffenreichweite hinaus).
const FOCUS_REACH: f32 = 7.0;
/// Mindestzahl gesunder Kameraden, damit sich Verwundete zurückziehen (sonst wird bis zum Ende gekämpft).
const MIN_HEALTHY: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Melee,
    Ranged,
    Support,
    Siege,
}

struct Own {
    id: UnitId,
    pos: Vec2,
    kind: Kind,
    range: f32,
    hp_ratio: f32,
    hero: bool,
    /// Kann fliegende Ziele treffen (Geschoss oder lange Waffe).
    anti_air: bool,
}

struct Foe {
    id: UnitId,
    pos: Vec2,
    class: UnitClass,
    hp_ratio: f32,
    flying: bool,
    speed: f32,
}

/// Gefahrenwert einer gegnerischen Klasse als Ziel (hoch = zuerst bekämpfen).
fn class_weight(c: UnitClass) -> f32 {
    match c {
        UnitClass::Support => 3.4,
        UnitClass::Siege => 3.0,
        UnitClass::Ranged => 2.8,
        UnitClass::Hero => 2.2,
        UnitClass::Cavalry => 1.4,
        UnitClass::Monster | UnitClass::Flyer => 1.3,
        UnitClass::Infantry => 1.0,
        UnitClass::Boss => 0.8,
        UnitClass::Worker => 0.15,
    }
}

fn unit_kind(c: UnitClass, range: f32) -> Option<Kind> {
    Some(match c {
        UnitClass::Worker | UnitClass::Boss | UnitClass::Flyer => return None,
        UnitClass::Siege => Kind::Siege,
        UnitClass::Support => Kind::Support,
        UnitClass::Ranged => Kind::Ranged,
        _ if range >= 3.5 => Kind::Ranged,
        _ => Kind::Melee,
    })
}

fn centroid(points: impl Iterator<Item = Vec2>) -> Option<Vec2> {
    let (sum, n) = points.fold((Vec2::ZERO, 0usize), |(s, n), p| (s + p, n + 1));
    (n > 0).then(|| sum / n as f32)
}

fn set_order(world: &mut SimWorld, id: UnitId, order: Order) {
    if let Some(u) = world.units.get_mut(id) {
        commands::set_order(u, order, false);
    }
}

fn attacking(o: Order) -> Option<EntityRef> {
    match o {
        Order::Attack { target } => Some(target),
        _ => None,
    }
}

/// Befiehlt eine Bewegung nur, wenn die Einheit nicht schon dorthin unterwegs ist.
fn move_to(world: &mut SimWorld, id: UnitId, target: Vec2, attack_move: bool, tolerance: f32) {
    let Some(u) = world.units.get(id) else { return };
    let already = matches!(u.order, Order::Move { target: t, attack_move: a } if a == attack_move && (t - target).length() <= tolerance);
    if already || (u.order == Order::Hold && (u.pos - target).length() <= tolerance) {
        return;
    }
    if !attack_move && (u.pos - target).length() <= 1.2 {
        if u.order != Order::Hold {
            set_order(world, id, Order::Hold);
        }
        return;
    }
    set_order(world, id, Order::Move { target, attack_move });
}

/// Führt die Seite `side` einen Taktikschritt lang. Gegner ist jeweils die andere Seite (Spieler gegen Feind).
pub fn drive(world: &mut SimWorld, data: &GameData, side: PlayerId, role: Role) {
    let foe_owner = if side == ENEMY { PLAYER } else { ENEMY };
    let mut own: Vec<Own> = Vec::new();
    for (id, u) in world.units.iter() {
        if u.owner != side || u.hp <= 0.0 || u.master.is_some() || u.routing > 0.0 {
            continue;
        }
        let d = data.unit(u.kind);
        if d.damage_max <= 0.0 && d.class != UnitClass::Support {
            continue;
        }
        let Some(kind) = unit_kind(d.class, d.range) else { continue };
        own.push(Own { id, pos: u.pos, kind, range: d.range + d.radius, hp_ratio: u.hp / u.max_hp.max(1.0), hero: d.class == UnitClass::Hero, anti_air: d.projectile.is_some() || d.range >= 1.0 });
    }
    if own.is_empty() {
        return;
    }
    let foes: Vec<Foe> = world
        .units
        .iter()
        .filter(|(_, u)| u.owner == foe_owner && u.hp > 0.0)
        .map(|(id, u)| {
            let d = data.unit(u.kind);
            Foe { id, pos: u.pos, class: d.class, hp_ratio: u.hp / u.max_hp.max(1.0), flying: d.flying, speed: d.speed }
        })
        .collect();
    let foe_buildings: Vec<(BuildingId, Vec2, Option<Fort>, bool)> = world
        .buildings
        .iter()
        .filter(|(_, b)| b.owner == foe_owner && b.hp > 0.0 && !b.invulnerable)
        .map(|(id, b)| (id, b.center(), data.building(b.kind).fort, b.tag.is_some()))
        .collect();
    let fighters: Vec<&Foe> = foes.iter().filter(|f| f.class != UnitClass::Worker).collect();
    let foe_c = centroid(fighters.iter().map(|f| f.pos))
        .or_else(|| centroid(foes.iter().map(|f| f.pos)))
        .or_else(|| centroid(foe_buildings.iter().map(|b| b.1)));
    let Some(foe_c) = foe_c else { return };
    let melee_c = centroid(own.iter().filter(|o| o.kind == Kind::Melee).map(|o| o.pos));
    let front = melee_c.or_else(|| centroid(own.iter().map(|o| o.pos))).unwrap_or(foe_c);
    let default_back = if side == ENEMY { Vec2::new(1.0, 1.0) } else { Vec2::new(-1.0, -1.0) }.normalize();
    let back = (front - foe_c).try_normalize().unwrap_or(default_back);
    let lateral = Vec2::new(-back.y, back.x);
    let healthy = own.iter().filter(|o| o.hp_ratio >= 0.5).count();
    let melee_count = own.iter().filter(|o| o.kind == Kind::Melee).count();

    // Wehranlage der verteidigenden Seite.
    let forts: Vec<(Vec2, Option<Fort>, bool)> = world
        .buildings
        .iter()
        .filter(|(_, b)| b.owner == side && b.hp > 0.0 && b.complete)
        .filter_map(|(_, b)| {
            let d = data.building(b.kind);
            (d.is_fort() || d.attack.is_some()).then(|| (b.center(), d.fort, d.attack.is_some()))
        })
        .collect();
    if role == Role::Defender && !forts.is_empty() && defend_fortress(world, data, side, &own, &foes, &forts, foe_c) {
        return;
    }

    // Wer greift wen an (für das Fokusfeuer: Ziele, die Kameraden schon bekämpfen, werden bevorzugt).
    let mut focus_count: Vec<(UnitId, u32)> = Vec::new();
    for o in &own {
        if let Some(EntityRef::Unit(t)) = world.units.get(o.id).and_then(|u| attacking(u.order)) {
            match focus_count.iter_mut().find(|(i, _)| *i == t) {
                Some((_, n)) => *n += 1,
                None => focus_count.push((t, 1)),
            }
        }
    }
    let score = |from: Vec2, f: &Foe, focus: &[(UnitId, u32)]| -> f32 {
        let d = (f.pos - from).length();
        let sticky = 1.0 + (focus.iter().find(|(i, _)| *i == f.id).map_or(0, |(_, n)| *n) as f32 * 0.12).min(1.0);
        class_weight(f.class) * (1.0 + 2.0 * (1.0 - f.hp_ratio)) * sticky / (1.0 + d / 6.0)
    };
    let best_foe = |from: Vec2, reach: f32, att: &Own, focus: &[(UnitId, u32)]| -> Option<(UnitId, f32)> {
        let att_def_ok = |f: &Foe| !f.flying || att.anti_air;
        let mut best: Option<(UnitId, f32)> = None;
        for f in &foes {
            if (f.pos - from).length() > reach || !att_def_ok(f) {
                continue;
            }
            let s = score(from, f, focus);
            if best.is_none_or(|(_, b)| s > b) {
                best = Some((f.id, s));
            }
        }
        best
    };
    let foe_by_id = |id: UnitId| foes.iter().find(|f| f.id == id);
    let nearest_fighter = |p: Vec2| fighters.iter().map(|f| (f.pos - p).length_squared()).min_by(|a, b| a.total_cmp(b)).map(f32::sqrt);

    // Ziele für Belagerungswaffen: Tore vor Türmen vor Mauern vor sonstigen Gebäuden.
    let siege_target = |from: Vec2| -> Option<BuildingId> {
        let pick = |pred: &dyn Fn(Option<Fort>) -> bool| {
            foe_buildings.iter().filter(|b| pred(b.2)).min_by(|a, b| (a.1 - from).length_squared().total_cmp(&(b.1 - from).length_squared())).map(|b| b.0)
        };
        pick(&|f| f == Some(Fort::Gate)).or_else(|| pick(&|f| f == Some(Fort::Tower))).or_else(|| pick(&|f| f == Some(Fort::Wall))).or_else(|| pick(&|_| true))
    };
    let chase: Vec<Vec2> = if fighters.is_empty() { foes.iter().map(|f| f.pos).collect() } else { fighters.iter().map(|f| f.pos).collect() };
    let keep_pos = foe_buildings.iter().find(|b| b.3 && b.2.is_none()).map(|b| b.1).or_else(|| foe_buildings.iter().find(|b| b.2.is_none()).map(|b| b.1));

    let mut ranged_slot = 0usize;
    for o in &own {
        let Some(u) = world.units.get(o.id) else { continue };
        let (order, pos) = (u.order, u.pos);
        let near_foe = nearest_fighter(pos).unwrap_or(f32::MAX);

        // Rückzug: Angeschlagene verlassen die Front, solange genug Gesunde kämpfen.
        let wounded = o.hp_ratio < if o.hero { WOUNDED - 0.05 } else { WOUNDED };
        if wounded && o.kind != Kind::Siege && healthy >= MIN_HEALTHY && near_foe < 12.0 {
            let spot = front + back * (9.0 + (o.id.index % 5) as f32 * 0.7) + lateral * (((o.id.index % 7) as f32) - 3.0) * 0.8;
            move_to(world, o.id, spot, false, 2.0);
            continue;
        }

        match o.kind {
            Kind::Siege => {
                let current = attacking(order).filter(|t| world.entity_alive(*t));
                if let Some(EntityRef::Building(b)) = current
                    && world.buildings.get(b).is_some_and(|x| data.building(x.kind).is_fort() || foe_buildings.iter().all(|fb| fb.2.is_none()))
                {
                    continue;
                }
                if let Some(t) = siege_target(pos) {
                    set_order(world, o.id, Order::Attack { target: EntityRef::Building(t) });
                } else if let Some(&(id, _)) = best_foe(pos, o.range + FOCUS_REACH, o, &focus_count).as_ref() {
                    set_order(world, o.id, Order::Attack { target: EntityRef::Unit(id) });
                }
            }
            Kind::Support => {
                if role == Role::Attacker || near_foe < 25.0 {
                    let post = front + back * 2.0 + lateral * (((o.id.index % 3) as f32) - 1.0) * 1.2;
                    move_to(world, o.id, post, false, 3.0);
                }
            }
            Kind::Ranged => {
                let reach = o.range + 1.5;
                let target = best_foe(pos, reach, o, &focus_count);
                let pressed = melee_count >= 2 && fighters.iter().any(|f| f.class != UnitClass::Ranged && f.class != UnitClass::Support && (f.pos - pos).length() < 2.2 && f.speed > 0.0);
                if pressed && (pos - front).length() < 10.0 {
                    // Ausweichen hinter die eigenen Nahkämpfer.
                    let post = front + back * 3.0 + lateral * (((ranged_slot % 7) as f32) - 3.0) * 0.9;
                    ranged_slot += 1;
                    move_to(world, o.id, post, false, 1.5);
                } else if let Some((t, s)) = target {
                    keep_or_switch(world, o.id, order, EntityRef::Unit(t), s, &foe_by_id, &score, &focus_count);
                } else if role == Role::Attacker && melee_count > 0 {
                    let slot = ranged_slot;
                    ranged_slot += 1;
                    let post = front + back * (3.0 + (slot / 7) as f32 * 1.1) + lateral * (((slot % 7) as f32) - 3.0) * 0.9;
                    move_to(world, o.id, post, true, 2.5);
                } else if role == Role::Attacker {
                    advance(world, o.id, order, pos, &chase, keep_pos);
                }
            }
            Kind::Melee => {
                let target = best_foe(pos, o.range.max(1.5) + FOCUS_REACH, o, &focus_count);
                if let Some((t, s)) = target {
                    keep_or_switch(world, o.id, order, EntityRef::Unit(t), s, &foe_by_id, &score, &focus_count);
                } else if role == Role::Attacker {
                    advance(world, o.id, order, pos, &chase, keep_pos);
                }
            }
        }
    }
}

/// Zielwert eines Gegners aus Sicht einer Position (Fokusfeuer).
type ScoreFn<'a> = dyn Fn(Vec2, &Foe, &[(UnitId, u32)]) -> f32 + 'a;

/// Behält das aktuelle Ziel, außer ein anderes ist deutlich besser; sonst Angriffsauftrag auf das beste Ziel.
#[allow(clippy::too_many_arguments)]
fn keep_or_switch<'a>(world: &mut SimWorld, id: UnitId, order: Order, best: EntityRef, best_score: f32, foe_by_id: &dyn Fn(UnitId) -> Option<&'a Foe>, score: &ScoreFn, focus: &[(UnitId, u32)]) {
    let Some(pos) = world.units.get(id).map(|u| u.pos) else { return };
    if let Some(cur) = attacking(order) {
        if cur == best {
            return;
        }
        if let EntityRef::Unit(c) = cur
            && let Some(f) = foe_by_id(c)
            && score(pos, f, focus) * 1.3 >= best_score
        {
            return;
        }
    }
    set_order(world, id, Order::Attack { target: best });
}

/// Vormarsch auf den nächsten Gegner bzw. das gegnerische Hauptgebäude.
fn advance(world: &mut SimWorld, id: UnitId, order: Order, pos: Vec2, chase: &[Vec2], keep: Option<Vec2>) {
    let objective = chase.iter().min_by(|a, b| (**a - pos).length_squared().total_cmp(&(**b - pos).length_squared())).copied().or(keep);
    let Some(t) = objective else { return };
    if matches!(order, Order::Attack { .. }) {
        return;
    }
    move_to(world, id, t, true, 4.0);
}

/// Verteidigungsverhalten einer Seite mit Wehranlage. Gibt `true` zurück, wenn die Befehle vollständig erteilt sind;
/// `false` (Durchbruch oder Feind fern) überlässt der allgemeinen Taktik bzw. den Wachposten die Führung.
fn defend_fortress(world: &mut SimWorld, data: &GameData, side: PlayerId, own: &[Own], foes: &[Foe], forts: &[(Vec2, Option<Fort>, bool)], foe_c: Vec2) -> bool {
    let _ = (data, side);
    let wall_like: Vec<&(Vec2, Option<Fort>, bool)> = forts.iter().filter(|f| f.1.is_some()).collect();
    let basis: Vec<Vec2> = if wall_like.is_empty() { forts.iter().map(|f| f.0).collect() } else { wall_like.iter().map(|f| f.0).collect() };
    let Some(center) = centroid(basis.iter().copied()) else { return false };
    let radius = (basis.iter().map(|p| (*p - center).length()).sum::<f32>() / basis.len() as f32).max(4.0);
    let combatants: Vec<&Foe> = foes.iter().filter(|f| f.class != UnitClass::Worker).collect();
    let Some(nearest) = combatants.iter().map(|f| (f.pos - center).length()).min_by(f32::total_cmp) else { return false };
    if nearest > radius + 18.0 {
        // Feind noch weit: Wachposten halten.
        return true;
    }
    let inside = combatants.iter().filter(|f| (f.pos - center).length() < radius * 0.7 && !f.flying).count();
    if inside >= 2 {
        // Durchbruch: alle in den Nahkampf (allgemeine Taktik übernimmt).
        return false;
    }
    let toward = (foe_c - center).try_normalize().unwrap_or(Vec2::new(-1.0, -1.0).normalize());
    // Posten auf Türmen, Toren und Mauerabschnitten, nach Nähe zum Feind sortiert.
    let mut posts: Vec<Vec2> = forts
        .iter()
        .filter(|f| f.2 || f.1 == Some(Fort::Tower) || f.1 == Some(Fort::Gate))
        .map(|f| f.0 + (center - f.0).try_normalize().unwrap_or(Vec2::ZERO) * 2.4)
        .collect();
    if posts.is_empty() {
        posts = forts.iter().map(|f| f.0 + (center - f.0).try_normalize().unwrap_or(Vec2::ZERO) * 2.4).collect();
    }
    posts.sort_by(|a, b| (*a - foe_c).length_squared().total_cmp(&(*b - foe_c).length_squared()));
    let manned = posts.len().clamp(1, 8);
    let choke = center + toward * radius * 0.6;
    let (mut r, mut m) = (0usize, 0usize);
    for o in own {
        let Some(u) = world.units.get(o.id) else { continue };
        let (order, pos) = (u.order, u.pos);
        match o.kind {
            Kind::Ranged | Kind::Siege => {
                let slot = r;
                r += 1;
                let post = posts[slot % manned] + Vec2::new(-toward.y, toward.x) * (slot / manned) as f32 * 0.9;
                if (pos - post).length() > 1.4 {
                    let engaged = nearest_dist(&combatants, pos) < 9.0;
                    move_to(world, o.id, post, engaged, 1.0);
                } else if order != Order::Hold {
                    set_order(world, o.id, Order::Hold);
                }
            }
            Kind::Support => {
                let post = center - toward * radius * 0.3;
                move_to(world, o.id, post, false, 3.0);
            }
            Kind::Melee => {
                let spread = Vec2::new(-toward.y, toward.x) * (((m % 5) as f32) - 2.0) * 0.8;
                m += 1;
                let post = choke + spread;
                let keeps = matches!(order, Order::Guard { post: p, .. } if (p - post).length() < 1.6);
                if !keeps {
                    set_order(world, o.id, Order::Guard { post, radius: 7.0 });
                }
            }
        }
    }
    true
}

fn nearest_dist(foes: &[&Foe], p: Vec2) -> f32 {
    foes.iter().map(|f| (f.pos - p).length()).min_by(f32::total_cmp).unwrap_or(f32::MAX)
}
