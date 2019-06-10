use wasm_bindgen::prelude::*;

use humus::{
    node::Element,
    render::{h, t},
    vdom::VirtualDom,
};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Called by our JS entry point to run the example.
#[wasm_bindgen(start)]
pub fn run() {
    // Set up the panic hook for debugging when things go wrong.
    set_panic_hook();

    // Grab the document's `<body>`.
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();
    let body = document.body().unwrap_throw();
    let root: Element = wasm_bindgen::JsCast::dyn_into::<web_sys::Element>(body)
        .unwrap()
        .into();

    let mut vd = VirtualDom::new();

    vd.render(
        root,
        h(
            "div",
            vec![
                h("h1", vec![t("Hello")]),
                h("h1", vec![t("World")]),
                h("h1", vec![t("Wasm")]),
            ],
        ),
    );

    // Run the virtual DOM forever and don't unmount it.
    //vdom.forget();
}

fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
