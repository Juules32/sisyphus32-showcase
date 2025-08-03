use dioxus::prelude::*;

use crate::components::{ChessBoard, Layout, Lorem};

#[component]
pub fn ChessEngines() -> Element {
    rsx! {
        Layout { 
            h1 { "Chess Engines" },
            p {
                "
                    Chess programming is a particular interest of mine, as it combines many
                    interesting areas of programming, including algorithmic and low-level optimization,
                    stochastic variability, and lots and lots of bitwise operations.
                "
            },
            h2 { "JuulesPlusPlus" },
            p {
                "
                    The first engine I created, I ingeniously named JuulesPlusPlus, because it was
                    written in C++. Everything was written from the bottom up, and it was my introduction
                    to the language. The main search function uses alpha-beta pruning with many heuristics
                    and optimizations on top, including magic bitboards, piece-square tables, iterative
                    deepening, quiescence search, PV moves, null-move pruning, the killer heuristic, and more.
                    I was very happy with the final result, however, there were a few
                    problems stemming from the architecture of the source code, which made it hard to
                    debug and hard to performance test.
                "
            },
            h2 { "Sisyphus32" },
            p {
                "
                    ... is the name of a chess engine I developed in Rust as part of 
                    my Bachelor's thesis on chess engine optimization and performance testing.
                    It fixes many of the issues I had with JuulesPlusPlus and adds many new features
                    such as zobrist hashing, an opening book, endgame tablebases, and parallelization
                    with Lazy SMP. Below, I've included a playable demo.
                    Due to the performance limitations of WASM, it is not nearly at full strength, but
                    is nonetheless a very challenging opponent!
                    Alternatively, you might be able to challenge Sisyphus32 on
                "
                Link {
                    to: "https://lichess.org/@/Sisyphus32",
                    "Lichess"
                }
                "
                    , where it has a bot profile.
                    Additionally, the engine has been published as a standalone library on     
                "
                Link {
                    to: "https://crates.io/crates/sisyphus32",
                    "crates.io"
                }
                ", which currently has more than 2000 downloads."
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
