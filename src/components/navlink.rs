use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn NavLink(to: Route, label: &'static str, is_outer: bool) -> Element {
    let current_route: Route = use_route();
    let is_active = current_route == to
        || (is_outer && to.to_string() != "/" && current_route.to_string().contains(&to.to_string().split("/").nth(1).unwrap()));

    rsx! {
        Link {
            to: to,
            class: if is_active {
                "navlink active"
            } else {
                "navlink"
            },
            {label}
        }
    }
}
