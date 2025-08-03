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
                    build hobby pages. This way, I've garnered experience in many web technologies.
                "
            }
            
            h2 { "PokéLink" }

            div {
                id: "pokelink",
                p {
                    "
                        PokéLink is a Pokémon-inspired browser game in the \"dle\" genre
                        (wordle, globle, etc.), meaning one new puzzle is generated automatically
                        every day.
                        The goal is to get from one Pokémon to another following a set of rules.
                        The frontend is written in Svelte + Tailwind + TypeScript, and the backend,
                        which responsible for retrieving Pokémon data, generating puzzles, 
                        and verifying guesses, among other things, is written in Python and run
                        with FastAPI. Puzzle and user data is stored in a simple Postgres database,
                        while puzzle progress is stored in the browser's local storage.
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
                iframe { src: "https://pokelink.juules32.com/" }
            }

        }
    }
}
