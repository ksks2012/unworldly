use rltk::{VirtualKeyCode, Rltk};
use super::{State, RunState};

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => { return RunState::AwaitingInput } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 => {},

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
    RunState::PlayerTurn
}
