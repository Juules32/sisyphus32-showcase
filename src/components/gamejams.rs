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
                    days, so bugs will be plentiful... 🐛
                "
            }
            div {
                class: "gamejam-entry",
                iframe {
                    id: "itch-widget",
                    frame_border: "0",
                    src: "https://itch.io/embed/3451248",
                    a {
                        href: "https://https://juules32.itch.io/potato-protecc",
                        "Potato Protecc by juules32"
                    }
                }
                p {
                    class: "gamejam-entry-text",
                    "
                        Developed in Godot as part of Nordic Game Jam 2025, in Potato Protecc, you are a starving
                        farmer who wants to grow potatoes. However, plant infestations are rampant,
                        and mice eat away at your crops. In addition to the core gameplay loop, the game
                        offers pixel art graphics, a level selection screen, background music, and
                        a high score system. 
                    "
                }
            }
            div {
                class: "gamejam-entry",
                iframe {
                    id: "itch-widget",
                    frame_border: "0",
                    src: "https://itch.io/embed/2659991",
                    a {
                        href: "https://https://juules32.itch.io/braininvader",
                        "BrainInvader by juules32"
                    }
                }
                p {
                    class: "gamejam-entry-text",
                    "
                        I developed BrainInvader in Godot during Nordic Game Jam 2024. It's a simple game
                        where the objective is to survive for as long as possible by creating and
                        traversing your own network of memories while conquering bad thoughts.
                    "
                }
            }
        }
    }
}
