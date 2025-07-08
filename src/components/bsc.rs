use dioxus::prelude::*;

use crate::components::{Layout, Lorem};

#[component]
pub fn BSc() -> Element {
    rsx! {
        Layout { 
            h1 { "BSc!" }
            Lorem {}
        }
    }
}
