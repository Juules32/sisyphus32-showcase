mod components;

use components::{ChessBoard, MatchSettings};
use dioxus::prelude::*;
use dioxus_logger::tracing;

const FAVICON: Asset = asset!("/assets/piece_svg/WN.svg");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(|| {
        tracing::debug!("Rendering app!");
        App()
    });
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Showcase {}
    }
}

#[component]
fn Showcase() -> Element {
    rsx! {
        div {
            class: "showcase",
            ChessBoard {}
            div {
                class: "text-column",
                h2 { class: "match-settings-heading", "Sisyphus32 Match settings" }
                MatchSettings {}
                hr { class: "text-separator" }
                p {
                    "
                        Sisyphus32 is a chess engine written in Rust, developed as part of a Bachelor's
                        thesis on chess engine optimization and performance testing. It features alpha-beta
                        pruning with magic bitboards, zobrist hashing, an opening book, endgame tablebases,
                        parallelization with Lazy SMP, and many more features. Due to the performance limitations of WASM, this
                        demo doesn't play at full strength, but is nonetheless a challenging opponent!
                        You can also challenge Sisyphus32 on "
                    a {
                        href: "https://lichess.org/@/Sisyphus32",
                        target: "_blank",
                        "Lichess"
                    }
                    ", where it has a bot profile, or use it as a standalone library from "
                    a {
                        href: "https://crates.io/crates/sisyphus32",
                        target: "_blank",
                        "crates.io"
                    }
                    "."
                }
            }
        }
    }
}
