use dioxus::prelude::*;

use crate::components::{Layout, Lorem};

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        Layout { 
            h1 { "Welcome to my page!" },
            Lorem {}
        }
    }
}
