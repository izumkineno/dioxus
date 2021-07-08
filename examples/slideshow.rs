//! Example: Webview Renderer
//! -------------------------
//!
//! This example shows how to use the dioxus_webview crate to build a basic desktop application.
//!
//! Under the hood, the dioxus_webview crate bridges a native Dioxus VirtualDom with a custom prebuit application running
//! in the webview runtime. Custom handlers are provided for the webview instance to consume patches and emit user events
//! into the native VDom instance.
//!
//! Currently, NodeRefs won't work properly, but all other event functionality will.

use dioxus::prelude::*;

fn main() {
    dioxus::desktop::launch(App, |c| c);
}

static App: FC<()> = |cx| {
    let slide_id = use_state(cx, || 0);

    let slide = match *slide_id {
        0 => cx.render(rsx!(Title {})),
        1 => cx.render(rsx!(Slide1 {})),
        2 => cx.render(rsx!(Slide2 {})),
        3 => cx.render(rsx!(Slide3 {})),
        _ => cx.render(rsx!(End {})),
    };

    cx.render(rsx! {
        div {
            div {
                div { h1 {"my awesome slideshow"} }
                div {
                    button {"<-", onclick: move |_| if *slide_id != 0 { *slide_id.get_mut() -= 1}}
                    h3 { "{slide_id}" }
                    button {"->" onclick: move |_| if *slide_id != 4 { *slide_id.get_mut() += 1 }}
                 }
            }
            {slide}
        }
    })
};

const Title: FC<()> = |cx| {
    cx.render(rsx! {
        div {

        }
    })
};
const Slide1: FC<()> = |cx| {
    cx.render(rsx! {
        div {

        }
    })
};
const Slide2: FC<()> = |cx| {
    cx.render(rsx! {
        div {

        }
    })
};
const Slide3: FC<()> = |cx| {
    cx.render(rsx! {
        div {

        }
    })
};
const End: FC<()> = |cx| {
    cx.render(rsx! {
        div {

        }
    })
};