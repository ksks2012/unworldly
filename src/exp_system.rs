
#![allow(unused)]
use std::time::Duration;

use rltk::console;

use crate::BiotechnologyState;
use specs::prelude::*;
use super::{ExpClock, ExpState, gamelog::GameLog, Name};

pub struct ExpSystem {}

impl<'a> System<'a> for ExpSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( 
                        Entities<'a>,
                        WriteStorage<'a, ExpClock>,
                        WriteStorage<'a, BiotechnologyState>,
                        ReadExpect<'a, Entity>, // The bio
                        WriteExpect<'a, GameLog>,
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut exp_clock, mut bio_state, bio_entity, mut log) = data;

        for (entity, mut clock, mut state) in (&entities, &mut exp_clock, &mut bio_state).join() {
            let mut proceed = false;
            
            clock.duration -= 1;
            if clock.duration < 1 {
                match clock.state {
                    ExpState::Training => {
                        clock.state = ExpState::Ready;
                    }
                    ExpState::Ready => {
                        clock.state = ExpState::Training;
                        clock.duration = 20;
                        // FIXME: entity != *bio_entity after load game
                        if state.exp < state.max_exp {
                            state.exp += 1;
                            console::log(&format!("Curent EXP: {}", state.exp));
                            log.entries.push(format!("Curent EXP: {}", state.exp));
                        } else {
                            clock.state = ExpState::Bottleneck;
                        }
                    }
                    ExpState::Bottleneck => {
                        // TODO: Breakthrough Chance
                        let chance = (100 - state.level);
                        let mut rng: rltk::prelude::RandomNumberGenerator = rltk::RandomNumberGenerator::new();
                        let rng_num = rng.roll_dice(1, 100);
                        if rng_num < chance {
                            state.level += 1;
                            state.max_exp = state.level * 10;
                            state.exp = 0;
                            console::log(&format!("Level Up! Level: {}", state.level));
                            log.entries.push(format!("Level Up! Level: {}", state.level));
                        } else {
                            console::log(&format!("Fail to break bottleneck! Level: {}", state.level));
                            log.entries.push(format!("Fail to break bottleneck! Level: {}", state.level));
                        }
                        clock.state = ExpState::Training;
                        clock.duration = 20;
                    }
                }
            }
        }
    }
}
