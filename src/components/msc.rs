use dioxus::prelude::*;

use crate::components::{Layout, Lorem};

#[component]
pub fn MSc() -> Element {
    rsx! {
        Layout { 
            h1 { "MSc!!!" }
            Lorem {}
        }
    }
}
