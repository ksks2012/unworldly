use rltk::{VirtualKeyCode, Rltk, console};
use super::{State, RunState};

pub fn player_input(_gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => { return RunState::AwaitingInput } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 => {
                console::log(&format!("Left"));
            },

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 => {},

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 => {},

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 => {},

            // Save and Quit
            VirtualKeyCode::Escape => return RunState::SaveGame,

            _ => { return RunState::AwaitingInput }
        },
    }
    console::log(&format!("PlayerTurn"));
    RunState::PlayerTurn
}
