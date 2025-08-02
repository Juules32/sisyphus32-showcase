use dioxus::prelude::*;

use crate::components::{Layout, Copy};

const PROFILE_PHOTO: Asset = asset!("/assets/profile_photo.jpg");

#[component]
pub fn Home() -> Element {
    rsx! {
        Layout {
            h1 { "Hello There! 👋" }
            div {
                img {
                    id: "profile-photo",
                    src: PROFILE_PHOTO,
                    "da"
                }
                p {
                    "
                        I'm Benjamin aka. Juules32, and welcome to my portfolio website!
                        I treat my portfolio as a space where I collect and share some personal
                        projects, apps and games I've built. It's more of a digital playground
                        than a professional résumé, but I've poured real time and care into
                        the things here.
                        This SPA is written in 
                    "
                    Link {
                        to: "https://dioxuslabs.com",
                        "Dioxus"
                    }
                    ", a Rust-based web framework."
                }
                p {
                    "
                        In 2025, I graduated from the IT University of Copenhagen with a Bachelor's
                        in software development.
                        Now, I'm looking forward to starting my Master's degree in Games, also at ITU.
                        I'm particularly interested in low-level optimization, software architecture,
                        and algorithmic problem solving. When I'm not programming, I like playing
                        physical and digital games, reading, and recently, a bit of photography.
                    "
                }
                p {
                    "
                        To get in touch, find me on 
                    " 
                    Link {
                        to: "https://github.com/Juules32",
                        "GitHub"
                    }
                    " and "
                    Link {
                        to: "https://www.linkedin.com/in/benjamin-jensen-476701373/",
                        "LinkedIn"
                    }
                    ", or contact me by mail: "
                    Copy {
                        text: "Juules32@gmail.com"
                    }
                    ". "
                }
            }
        }
    }
}
