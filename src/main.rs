use rltk::{GameState, Rltk};
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

mod biotechnology;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod gamelog;
mod gui;
mod player;
pub use player::*;
mod saveload_system;
mod spawner;
pub use spawner::*;
pub mod exp_system;

pub struct State {
    pub ecs: World
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { 
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    MainMenu { menu_selection : gui::MainMenuSelection },
    SaveGame
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
        let mut exp = exp_system::ExpSystem{};
        exp.run_now(&self.ecs);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        ctx.cls();

        match newrunstate {
            RunState::MainMenu{..} => {}
            _ => {
                {
                    self.run_systems();

                    let map = self.ecs.fetch::<Vec<TileType>>();
                    draw_map(&map, ctx);

                    let positions = self.ecs.read_storage::<Position>();
                    let renderables = self.ecs.read_storage::<Renderable>();

                    for (pos, render) in (&positions, &renderables).join() {
                        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                    }

                    gui::draw_ui(&self.ecs, ctx);
                }
            }
        }

        match newrunstate {
            RunState::PreRun => {
                saveload_system::delete_save();
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                // Player input
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::MainMenu{ .. } => {
                let result = gui::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection{ selected } => newrunstate = RunState::MainMenu{ menu_selection: selected },
                    gui::MainMenuResult::Selected{ selected } => {
                        match selected {
                            gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                            gui::MainMenuSelection::LoadGame => {
                                // Load game
                                saveload_system::load_game(&mut self.ecs);
                                newrunstate = RunState::AwaitingInput;
                            }
                            gui::MainMenuSelection::Quit => { ::std::process::exit(0); }
                        }
                    }
                }
            }
            RunState::SaveGame => {
                saveload_system::save_game(&mut self.ecs);
                newrunstate = RunState::MainMenu{ menu_selection : gui::MainMenuSelection::LoadGame };
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }

    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let mut context = RltkBuilder::simple80x50()
        .with_title("Unworldly")
        .build()?;
    context.with_post_scanlines(true);
    let mut gs: State = State {
        ecs: World::new(),
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Biotechnology>();
    gs.ecs.register::<BiotechnologyState>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<ExpClock>();
    gs.ecs.register::<SimpleMarker<SerializeMe>>();

    // NOTE: Marker is inserted before other entries
    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    let bio_entity = spawner::biotechnology(&mut gs.ecs);

    gs.ecs.insert(new_map());
    gs.ecs.insert(bio_entity);
    gs.ecs.insert(RunState::MainMenu{ menu_selection: gui::MainMenuSelection::NewGame });
    gs.ecs.insert(gamelog::GameLog{ entries : vec!["Welcome to Unworldly".to_string()] });

    rltk::main_loop(context, gs)
}
