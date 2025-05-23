use dioxus::prelude::*;

use crate::components::{Layout, Lorem};

/// Home page
#[component]
pub fn BSc() -> Element {
    rsx! {
        Layout { 
            h1 { "BSc!" }
            Lorem {}
        }
    }
}
