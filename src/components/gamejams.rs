use dioxus::prelude::*;

use crate::components::Layout;

#[component]
pub fn GameJams() -> Element {
    rsx! {
        Layout {
            h1 { "Game Jams" }
            p {
                "
                    Over the years, I have participated in a few game jams which are listed below.
                    Keep in mind all of these were made from start to finish in the span of a few
                    days, so you might encounter a few bugs...
                "
            }
            div {
                id: "gamejam-entry",
                iframe {
                    id: "itch-widget",
                    frame_border: "0",
                    src: "https://itch.io/embed/3451248",
                    a {
                        href: "https://https://juules32.itch.io/potato-protecc",
                        "Potato Protecc by juules32"
                    }
                }
                a {
                    "Template game jam text..."
                }
            }

            div {
                id: "gamejam-entry",
                iframe {
                    id: "itch-widget",
                    frame_border: "0",
                    src: "https://itch.io/embed/2659991",
                    a {
                        href: "https://https://juules32.itch.io/braininvader",
                        "BrainInvader by juules32"
                    }
                }
                a {
                    "Template game jam text..."
                }
            }
        }
    }
}
