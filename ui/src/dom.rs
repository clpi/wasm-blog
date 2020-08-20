use wasm_bindgen::prelude::*;
use std::collections::btree_map::*;
use crate::parse::ParseType;
use web_sys::{Document, Element, HtmlElement, Window, console};
use js_sys::{Promise, Reflect, WebAssembly, Uint8ClampedArray};

pub mod elements;


pub struct Dom {
    pub body: BTreeMap<&'static str, Box<dyn Elem>>, 
}

impl Dom {

    pub fn new() -> Self {
        Dom {
            body: BTreeMap::new(),
        }
    }

    pub fn div(&mut self, id: &'static str, content: &'static dyn FnMut() -> Vec<Box<dyn Elem>>) -> () {
    }
}

pub struct Div {
    pub children: Vec<Box<dyn Elem>>,
}

impl Div {
    pub fn new(children: &'static dyn FnMut() -> Vec<Box<dyn Elem>>) -> () {
    }
}

impl Elem for Div {

}

pub struct Node {
    pub children: Vec<Self>,
    pub entry: ParseType,
}

impl Node {
    pub fn new(node: ParseType) -> Self {
        Self { children: Vec::new(), entry: node }
    }
}

pub trait Elem {}
