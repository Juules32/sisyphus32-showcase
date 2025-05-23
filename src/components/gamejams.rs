use dioxus::prelude::*;

use crate::components::{Layout, Lorem};

#[component]
pub fn GameJams() -> Element {
    rsx! {
        Layout {
            h1 { "Game Jams!" }
            Lorem {}
        }
    }
}
