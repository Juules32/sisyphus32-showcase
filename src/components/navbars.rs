use dioxus::prelude::*;
use crate::{components::{navlink::NavLink, Layout}, Route};

#[component]
pub fn OuterNavbar() -> Element {
    rsx! {
        div {
            class: "navbar outer",
            NavLink { to: Route::Home {}, label: "Home", is_outer: true }
            NavLink { to: Route::AboutApps {}, label: "Apps", is_outer: true }
        }

        Outlet::<Route> {}
    }
}

#[component]
pub fn AppsNavbar() -> Element {
    rsx! {
        div {
            class: "navbar inner",
            NavLink { to: Route::AboutApps {}, label: "About", is_outer: false }
            NavLink { to: Route::PokeLink {}, label: "PokéLink", is_outer: false }
            NavLink { to: Route::WordGuessr {}, label: "WordGuessr", is_outer: false }
        }

        Outlet::<Route> {}
    }
}
