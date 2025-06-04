use dioxus::prelude::*;
use crate::Route;

#[derive(Props, PartialEq, Clone)]
pub struct NavLinkProps {
    to: Route,
    label: &'static str,
    is_outer: bool
}

#[component]
pub fn NavLink(props: NavLinkProps) -> Element {
    let current_route: Route = use_route();
    let is_active = current_route == props.to
        || (props.is_outer && props.to.to_string() != "/" && current_route.to_string().contains(&props.to.to_string().split("/").nth(1).unwrap()));

    rsx! {
        Link {
            to: props.to,
            class: if is_active {
                "navlink active"
            } else {
                "navlink"
            },
            {props.label}
        }
    }
}
