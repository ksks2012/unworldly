use rltk::{GameState, Rltk, Point};
use specs::prelude::*;

mod biotechnology;
mod camera;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod gamelog;
mod gui;
mod spawner;
pub use spawner::*;
pub mod exp_system;

pub struct State {
    pub ecs: World
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
        ctx.cls();

        self.run_systems();

        camera::render_camera(&self.ecs, ctx);
        gui::draw_ui(&self.ecs, ctx);
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
    gs.ecs.register::<Hidden>();

    let bio_entity = spawner::biotechnology(&mut gs.ecs);

    gs.ecs.insert(Map::new(1, 64, 64));
    gs.ecs.insert(Point::new(0, 0));
    gs.ecs.insert(bio_entity);
    gs.ecs.insert(gamelog::GameLog{ entries : vec!["Welcome to Unworldly".to_string()] });

    rltk::main_loop(context, gs)
}
