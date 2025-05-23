use dioxus::prelude::*;

use crate::components::Layout;

#[component]
pub fn WebApps() -> Element {
    rsx! {
        Layout {
            iframe { id: "pokelink", src: "https://pokelink.juules32.com/" }
            iframe { id: "wordguessr", src: "https://wordguessr.juules32.com/" }
        }
    }
}
