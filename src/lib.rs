use wasm_bindgen::prelude::*;
use gloo::{events::EventListener};
mod damn;

// run on load
#[wasm_bindgen]
pub fn setup() {

    // setup dom
    let window: web_sys::Window = web_sys::window().expect("");    
    let document: web_sys::Document = window.document().expect("");
    
    // attach elements
    let outp_header: web_sys::Element = document.get_element_by_id("subtitle").unwrap();
    let main_button: web_sys::Element = document.get_element_by_id("main_button").unwrap();
    let slider_divs: web_sys::Element = document.get_element_by_id("position").unwrap();
    
    // attach button listener
    let button_listener: EventListener = EventListener::new(&main_button, "click", move |_event| {
        outp_header.set_inner_html(&damn::test_add(1, 2).to_string()[..]);
    });
    button_listener.forget(); 
    
    // add sliders  
    let mut sliders: Vec<web_sys::Element> = vec![];
    for index in 0..10 {
        let base_slider: web_sys::Element = document.create_element("input").unwrap();
        base_slider.set_attribute("type", "range").unwrap(); 
        base_slider.set_attribute("orient", "vertical").unwrap(); 
        base_slider.set_attribute("min", "1").unwrap(); 
        base_slider.set_attribute("value", "1").unwrap(); 
        base_slider.set_attribute("max", "3").unwrap(); 
        sliders.push(base_slider);
        slider_divs.append_child(&sliders[index]).unwrap();
    }
    /*for _ in 0..2 {
        slider_divs.append_child(&document.create_element("br").unwrap()).unwrap();
    }*/
}
