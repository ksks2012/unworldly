use specs::prelude::*;
use specs::saveload::{MarkedBuilder, SimpleMarker};

use super::{BiotechnologyState, Biotechnology, ExpState, ExpClock, Name, SerializeMe};

/// Spawns the Biotechnology and returns entity object.
pub fn biotechnology(ecs : &mut World) -> Entity {
    ecs
        .create_entity()
        .with(Biotechnology{})
        .with(Name{name: "001".to_string() })
        .with(BiotechnologyState{ lifespan: 60, life: 0, level: 0, max_exp: 10, exp: 0 })
        .with(ExpClock{ state: ExpState::Training, duration: 20 })
        .marked::<SimpleMarker<SerializeMe>>()
        .build()
}