use dioxus::prelude::*;

use crate::components::Layout;

#[component]
pub fn WordGuessr() -> Element {
    rsx! {
        Layout {
            iframe { src: "https://wordguessr.juules32.com/" }
        }
    }
}
