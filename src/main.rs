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
        #[nest("/itu")]
            #[redirect("/", || Route::BSc {})]
            #[layout(ItuNavbar)]
                #[route("/bsc")]
                BSc {},
                #[route("/msc")]
                MSc {},
            #[end_layout]
        #[end_nest]
        #[nest("/projects")]
            #[redirect("/", || Route::WebApps {})]
            #[layout(ProjectsNavbar)]
                #[route("/apps")]
                WebApps {},
                #[route("/jams")]
                GameJams {},
                #[route("/chess")]
                ChessEngines {},
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    dioxus::launch(|| {
        tracing::debug!("Rendering app!");
        App()
    });
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
        Layout {
            h2 { "Route: \"{route_string}\" not found!" }
            Link { to: "/", "Go to home page" }
        }
    }
}
