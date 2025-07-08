use dioxus::prelude::*;
use web_sys::window;
use gloo_timers::future::TimeoutFuture;

#[component]
pub fn Copy(text: &'static str) -> Element {
    let mut indicator_visible = use_signal(|| false);
    let mut initial_indicator_hide = use_signal(|| true);

    spawn(async move {
        TimeoutFuture::new(300).await;
        initial_indicator_hide.set(false);
    });

    rsx! {
        a {
            class: "copy",
            onclick: move |_| {
                if let Some(window) = window() {
                    let _promise = window.navigator().clipboard().write_text(text);
                    
                    if !indicator_visible() {
                        spawn(async move {
                            TimeoutFuture::new(700).await;
                            indicator_visible.set(false);
                        });
                    }

                    indicator_visible.set(true);
                }
            },
            span {
                class: if indicator_visible() {"copy-indicator visible"} else {"copy-indicator"},
                style: if initial_indicator_hide() {"display: none;"} else {""},
                "Copied ✅"
            }
            span {
                class: "copy-text",
                "{text}"
            }
            "📋"
        }
    }
}
