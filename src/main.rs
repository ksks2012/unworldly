use rltk::{GameState, Rltk};
use specs::prelude::*;

mod biotechnology;
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

    let bio_entity = spawner::biotechnology(&mut gs.ecs);

    gs.ecs.insert(new_map());
    gs.ecs.insert(bio_entity);
    gs.ecs.insert(gamelog::GameLog{ entries : vec!["Welcome to Unworldly".to_string()] });

    rltk::main_loop(context, gs)
}
