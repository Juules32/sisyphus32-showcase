use dioxus::prelude::*;

use crate::components::Layout;

#[component]
pub fn WebApps() -> Element {
    rsx! {
        Layout {
            h1 { "Web Apps" }
            p {
                "
                    In my free time, I like to explore new tools and technologies to
                    build hobby pages.
                "
            }
            
            h2 { "PokéLink" }

            div {
                id: "pokelink",
                p {
                    "
                        PokéLink is a Pokémon-inspired browser game in the \"dle\" genre
                        (wordle, globle, etc.) 
                    "
                }
                p {
                    id: "playbelow",
                    "You can play the game below or visit the website "
                    Link {
                        to: "https://pokelink.juules32.com/",
                        "here!"
                    }
                }
                p {
                    id: "playlink",
                    "You can play the game "
                    Link {
                        to: "https://pokelink.juules32.com/",
                        "here!"
                    }
                }
                iframe { src: "https://pokelink.juules32.com/", allowfullscreen: "" }
            }

        }
    }
}
