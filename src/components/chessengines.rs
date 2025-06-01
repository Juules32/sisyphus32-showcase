use dioxus::prelude::*;

use crate::components::{ChessBoard, Layout, Lorem};

#[component]
pub fn ChessEngines() -> Element {
    rsx! {
        Layout { 
            h1 { "Chess Engines" },
            p {
                "
                    Chess programming is a particular interest of mine...
                "
            },
            h2 { "JuulesPlusPlus" },
            Lorem {},
            h2 { "Sisyphus32" },
            p {
                "
                    ... is the name of the chess engine I developed in Rust as part of 
                    my Bachelor's thesis on chess engine optimization and performance testing.
                    It fixes many of the issues I had with JuulesPlusPlus and adds many new features
                    such as an opening book, endgame tablebases, and parallelization with Lazy SMP.
                    Below, I've included a playable demo.
                    Due to the performance limitations of WASM, it is not nearly at full strength, but
                    is nonetheless a very challenging opponent!
                    Alternatively, you might be able to challenge Sisyphus32 on
                "
                Link {
                    to: "https://lichess.org/@/Sisyphus32",
                    "Lichess"
                }
                ", where it has a bot profile."
            }
            div {
                class: "board-centerer",
                ChessBoard {}
            }
        }
    }
}

/*
I did a c++ engine
i ran into bugs
which is why i did my bachelor project to find a way to 
avoid bugs and make sure new features actually yield better performance
*/
