use dioxus::prelude::*;
use crate::components::Layout;

#[component]
pub fn PokeLink() -> Element {
    rsx! {
        Layout {
            iframe { src: "https://pokelink.juules32.com/" }
        }
    }
}
