
#![allow(unused)]
use rltk::console;

use crate::BiotechnologyState;
use specs::prelude::*;
use super::{ExpClock, ExpState, Name};

pub struct ExpSystem {}

impl<'a> System<'a> for ExpSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( 
                        Entities<'a>,
                        WriteStorage<'a, ExpClock>,
                        WriteStorage<'a, BiotechnologyState>,
                        ReadExpect<'a, Entity>, // The bio
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut exp_clock, mut bio_state, bio_entity) = data;

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
                        if entity == *bio_entity && state.exp < state.max_exp {
                            state.exp += 1;
                            console::log(&format!("Curent EXP: {}", state.exp));
                        }
                    }
                }
            }
        }
    }
}
