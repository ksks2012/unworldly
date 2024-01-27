use rltk::{ RGB, Rltk };
use specs::prelude::*;
use super::{gamelog::GameLog};

const BOX_Y : i32 = 29;
const BOX_H : i32 = 20;
const MSG_Y : i32 = 30;

pub fn draw_ui(ecs: &World, ctx : &mut Rltk) {
    ctx.draw_box(0, BOX_Y, 79, BOX_H, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));

    let log = ecs.fetch::<GameLog>();
    let mut y = MSG_Y;
    for s in log.entries.iter().rev() {
        if y < 49 { ctx.print(2, y, s); }
        y += 1;
    }
}