use dioxus::prelude::*;

use crate::components::Layout;

#[component]
pub fn WebApps() -> Element {
    rsx! {
        Layout {
            iframe { id: "pokelink", src: "https://pokelink.juules32.com/" }
        }
    }
}
