//! Aktualisierung des Kriegsnebels aus den Sichtweiten des Spielers.

use crate::data::GameData;
use crate::entity::PLAYER;
use crate::world::SimWorld;

pub fn update(world: &mut SimWorld, data: &GameData) {
    let mut reveals = Vec::with_capacity(world.units.len() + world.buildings.len());
    for (_, u) in world.units.iter() {
        if u.owner == PLAYER {
            reveals.push((u.pos, world.sight_of(data, u)));
        }
    }
    for (_, b) in world.buildings.iter() {
        if b.owner == PLAYER {
            let s = if b.complete { data.building(b.kind).sight } else { 3.0 };
            reveals.push((b.center(), s + b.radius()));
        }
    }
    world.fog.begin_update();
    for (p, r) in reveals {
        world.fog.reveal(p, r);
    }
    world.fog.end_update();
}
