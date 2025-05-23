use dioxus::prelude::*;
use crate::{components::{navlink::NavLink}, Route};

#[component]
pub fn OuterNavbar() -> Element {
    rsx! {
        div {
            class: "navbar outer",
            NavLink { to: Route::Home {}, label: "Home", is_outer: true }
            NavLink { to: Route::BSc {}, label: "ITU", is_outer: true }
            NavLink { to: Route::AboutProjects {}, label: "Personal Projects", is_outer: true }
        }

        Outlet::<Route> {}
    }
}

#[component]
pub fn ItuNavbar() -> Element {
    rsx! {
        div {
            class: "navbar inner",
            NavLink { to: Route::BSc {}, label: "BSc in Software Development", is_outer: false }
            NavLink { to: Route::MSc {}, label: "MSc in Games", is_outer: false }
        }

        Outlet::<Route> {}
    }
}

#[component]
pub fn ProjectsNavbar() -> Element {
    rsx! {
        div {
            class: "navbar inner",
            NavLink { to: Route::AboutProjects {}, label: "About", is_outer: false }
            NavLink { to: Route::WebApps {}, label: "Web Apps", is_outer: false }
            NavLink { to: Route::GameJams {}, label: "Game Jams", is_outer: false }
        }

        Outlet::<Route> {}
    }
}
