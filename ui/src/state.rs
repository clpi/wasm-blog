use std::{f64, rc::Rc, cell::RefCell, future::Future};
use wasm_bindgen::prelude::*;

#[derive(Default)]
pub struct State {
    pub string: String,
}

#[derive(Default)]
pub struct Context {
    pub state: Rc<RefCell<State>>,
}

impl Context {

    pub fn new() -> Self { Self::default() }
}

#[wasm_bindgen]
pub struct IncrementFuture {
    context: Rc<RefCell<State>>,
}

impl IncrementFuture {

}
