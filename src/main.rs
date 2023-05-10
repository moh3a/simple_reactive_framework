use leptos_reactive;
use wasm_bindgen;
use web_sys;

fn main() {
    mount(El::new("button"))
}

#[derive(Debug, Clone)]
pub struct El(Element);
