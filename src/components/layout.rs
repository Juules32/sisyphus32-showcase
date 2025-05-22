use dioxus::prelude::*;

#[component]
pub fn Layout(children: Element) -> Element {
    rsx! {
        div {
            id: "layout-background",
            div {
                id: "layout-content",
                {children}
            }
        }
    }
}
