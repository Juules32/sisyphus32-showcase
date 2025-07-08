use dioxus::prelude::*;

use crate::components::{Layout, Copy};

#[component]
pub fn Home() -> Element {
    rsx! {
        Layout { 
            h1 { "Hello There! 👋" },
            p {
                "
                    I'm Benjamin aka. Juules32, and this is my portfolio website 
                    with information about my studies and personal projects.
                    This website is written in 
                "
                Link {
                    to: "https://dioxuslabs.com",
                    "Dioxus"
                }
                ", a Rust-based web framework."
            }
            p {
                "
                    You can find me on 
                " 
                Link {
                    to: "https://github.com/Juules32",
                    "github"
                }
                " or contact me by mail: "
                Copy {
                    text: "Juules32@gmail.com"
                }
                ". "
            }
        }
    }
}
