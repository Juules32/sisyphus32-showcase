use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn OuterNavbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::AboutApps {}, "Apps" }
        }

        Outlet::<Route> {}
    }
}

/// Shared navbar component.
#[component]
pub fn AppsNavbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link { to: Route::AboutApps {}, "About" }
            Link { to: Route::PokeLink {}, "PokéLink" }
            Link { to: Route::WordGuessr {}, "WordGuessr" }
        }

        Outlet::<Route> {}
    }
}
