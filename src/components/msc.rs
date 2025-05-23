use dioxus::prelude::*;

use crate::components::Layout;

/// Home page
#[component]
pub fn MSc() -> Element {
    rsx! {
        Layout { 
            h1 { "MSc!!!" }
        }
    }
}
