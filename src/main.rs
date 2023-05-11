use std::ops::Deref;

use leptos_reactive::{
    self, create_effect, create_runtime, create_scope, create_signal, Scope, SignalGet,
    SignalUpdate,
};
use wasm_bindgen::{self, JsCast, JsValue};
use web_sys::{self, console, window, Element, Event, Window};

fn main() {
    mount(El::new("button"));
}

fn mount(root: El) {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    body.append_child(&root).unwrap();
}

#[derive(Debug, Clone)]
pub struct El(Element);

// Implement Deref trait on El
// Implementing the Deref trait allows you to customize the behavior of the dereference operator * (not to be confused with the multiplication or glob operator). By implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.
impl Deref for El {
    // use El type as if it was a DOM element
    // to implement additional methods on the native DOM element
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl El {
    pub fn new(tag_name: &str) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let el = document.create_element(tag_name).unwrap();
        Self(el)
    }
}
