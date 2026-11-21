//! Tageszeit- und Wetterübergänge als Ereignisse.

use crate::chronicle::ChronicleKind;
use crate::data::GameData;
use crate::events::SimEvent;
use crate::world::SimWorld;

pub fn tick(world: &mut SimWorld, data: &GameData, dt: f32) {
    let tr = world.env.tick(dt, data.rules.day_length, &mut world.rng);
    if tr.night_began {
        let n = world.env.night;
        world.events.push(SimEvent::NightBegins { night: n });
        if n == 1 {
            world.record(ChronicleKind::Omen, "Die erste Nacht senkte sich über Brennhollow. Im Nordosten glomm fahles Licht.");
        }
    }
    if tr.day_began {
        let d = world.env.day;
        world.events.push(SimEvent::DayBegins { day: d });
    }
    if tr.weather_changed {
        let w = world.env.weather;
        world.events.push(SimEvent::WeatherChanged { weather: w });
    }
    if tr.lightning {
        world.events.push(SimEvent::Lightning);
    }
}
