use dioxus::prelude::*;
use web_sys::window;
use gloo_timers::future::TimeoutFuture;
use dioxus_elements::input_data::MouseButton;

#[component]
pub fn Copy(text: &'static str) -> Element {
    let mut indicator_visible = use_signal(|| false);
    let mut initial_indicator_hide = use_signal(|| true);

    spawn(async move {
        TimeoutFuture::new(300).await;
        initial_indicator_hide.set(false);
    });

    let on_mouse_down = move |evt: MouseEvent| {
        // Check for left or middle button
        if evt.trigger_button() == Some(MouseButton::Primary) || evt.trigger_button() == Some(MouseButton::Auxiliary) {
            if let Some(window) = window() {
                let _promise = window.navigator().clipboard().write_text(text);

                if !indicator_visible() {
                    spawn({
                        async move {
                            TimeoutFuture::new(700).await;
                            indicator_visible.set(false);
                        }
                    });
                }

                indicator_visible.set(true);
            }
        }
    };

    rsx! {
        a {
            class: "copy",
            onmousedown: on_mouse_down,
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
