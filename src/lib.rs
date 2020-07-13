use wasm_bindgen::prelude::*;
use gloo::{events::EventListener};
mod test;

// run on load
#[wasm_bindgen]
pub fn setup() {

    // setup dom
    let window: web_sys::Window = web_sys::window().expect("");    
    let document: web_sys::Document = window.document().expect("");
    
    // attach elements
    let outp_header: web_sys::Element = document.get_element_by_id("outp_header").unwrap();
    let main_button: web_sys::Element = document.get_element_by_id("main_button").unwrap();

    // attach button listener
    let button_listener: EventListener = EventListener::new(&main_button, "click", move |_event| {
        outp_header.set_inner_html(&test::test_add(1, 2).to_string()[..]);
    });
    button_listener.forget();  
}
