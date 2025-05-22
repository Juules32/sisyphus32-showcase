mod components;

use components::*;
use dioxus::prelude::*;
use dioxus_logger::tracing;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(OuterNavbar)]
        #[route("/")]
        Home {},
        #[nest("/apps")]
            #[layout(AppsNavbar)]
            #[route("/")]
            AboutApps {},
            #[route("/pokelink")]
            PokeLink {},
            #[route("/wordguessr")]
            WordGuessr {},
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    tracing::debug!("Rendering app!");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    let route_string = route.join("/");
    rsx! {
        "Route: {route_string} not found!"
    }
}
