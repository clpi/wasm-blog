use wasm_bindgen::prelude::*;
use std::rc::Weak;
use crate::*;
use js_sys::{Promise, Reflect, WebAssembly, Uint8ClampedArray};
use web_sys::{
    HtmlElement, HtmlCanvasElement, HtmlInputElement,
    HtmlLinkElement, HtmlButtonElement, HtmlSelectElement,
    HtmlTextAreaElement,
};
use std::{
    collections::{HashMap, BTreeSet},
    rc::Rc, cell::{RefCell, RefMut}
};

pub type StyleRule = (&'static str, &'static str);
pub type Styles = HashMap<&'static str, &'static str>;

pub struct Style<E> {
    styles: Styles,
    element: Box<E>,
}

pub trait Element {

    fn style(prop: &str, val: &str) -> ();
}

pub trait Clickable<E>: Element {
    fn on_click<F>(&mut self, func: &'static dyn FnMut() -> ()) -> ();
}

pub struct Button(HtmlButtonElement);

impl Button {
    
}

pub struct Input(HtmlLinkElement);

pub struct Btn {
    el: web_sys::HtmlButtonElement,
    text: &'static str,
    style: String,
    onclick: FnMut() -> (),
}

impl Btn {

    pub fn on_click<F>(&mut self, mut func: F) -> () where
    F: FnMut() -> () + 'static {
       let func = Closure::wrap(Box::new(move || { 
           func() 
       }) as Box<dyn FnMut()>) ;
       self.el.set_onclick(Some(func.as_ref().unchecked_ref()));
       func.forget();
    }

    pub fn style(&mut self, prop: &str, val: &str) -> () {
        self.el.style()
            .set_property(prop, val)
            .unwrap();
    }
}

pub trait Event {  }

pub struct KeyEvent {
   callback: Closure<dyn FnMut(web_sys::Event)>, 
}

pub fn div() -> web_sys::HtmlElement {
    let div = doc().create_element("div").expect("");
    doc().append_child(&div).unwrap();
    div.dyn_into::<HtmlElement>().unwrap()
}

pub fn btn(paren: Option<HtmlElement>, text: Option<&str>) -> web_sys::HtmlButtonElement {
    let btn = doc().create_element("btn").expect("");
    btn.set_text_content(text);
    if let Some(par) = paren {
        par.append_child(&btn).unwrap();
    } else {
        doc().append_child(&btn).unwrap();
    }
    btn.dyn_into::<web_sys::HtmlButtonElement>().unwrap()
}
