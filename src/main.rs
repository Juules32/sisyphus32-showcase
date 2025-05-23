mod components;

use components::*;
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(OuterNavbar)]
        #[route("/")]
        Home {},
        #[nest("/itu")]
            #[layout(ItuNavbar)]
                #[route("/bsc")]
                BSc {},
                #[route("/msc")]
                MSc {},
            #[end_layout]
        #[end_nest]
        #[nest("/projects")]
            #[layout(ProjectsNavbar)]
                #[route("/")]
                AboutProjects {},
                #[route("/apps")]
                WebApps {},
                #[route("/jams")]
                GameJams {},
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
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
        Layout {
            h2 { "Route: \"{route_string}\" not found!" }
            Link { to: "/", "Go to home page" }
        }
    }
}
