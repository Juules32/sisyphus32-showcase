use dioxus::prelude::*;

use crate::components::{Layout};

use sisyphus32::{BitMove, Legal, MoveGeneration, MoveList, Position};


/// Home page
#[component]
pub fn ChessEngines() -> Element {
    unsafe { sisyphus32::init() }
    let moves: MoveList<BitMove> = MoveGeneration::generate_moves::<BitMove, Legal>(&Position::starting_position());
    
    let first_move_info = format!("{}", moves.first());

    rsx! {
        Layout { 
            h1 { "Chess Engines" },
            {first_move_info}
        }
    }
}
