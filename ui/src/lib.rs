#[macro_use]
extern crate nom;

pub mod parse;
pub mod controller;
pub mod view;
pub mod model;

use std::mem::replace;
use futures_channel::oneshot;
use std::{f64, rc::Rc, cell::RefCell,};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{
    future_to_promise, JsFuture, spawn_local
};
use rand::{thread_rng, distributions::Alphanumeric, prelude::*};
use web_sys::{Document, Element, HtmlElement, Window, console};
use js_sys::{Promise, Reflect};
use nalgebra::*;

//use web_sys::console;


use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    fn log(s: &str);
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct State {
    pub string: String,
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    template().unwrap();

    let performance = window().performance().expect("");
    let start = get_time(performance.timing().request_start());
    let end = get_time(performance.timing().request_start());


    let func = Closure::wrap(Box::new(|| {
        println!("Hello")
    }) as Box<dyn FnMut()>);

    let mut state = State { string: "Hello".to_string() };

    let change: Closure<dyn FnMut(String) -> String>
        = Closure::once(move |s: String| {
            s
        });



    Ok(())
}

pub async fn operation() -> Result<JsValue, JsValue> {
    let val = async {
        Ok(JsValue::from_str("Ok"))
    };
    val.await
}

pub struct Elem { 
    el: Option<web_sys::Element>,
}

impl Elem {
    pub fn create(tag: &str) -> Option<Element> {
        match web_sys::window()?
            .document()?
            .create_element(tag).ok() {
            Some(el) => Some(el),
            None => None,
        }
        
    }

    pub fn select(select: &str) -> Option<Elem> {
        Some( Self { 
            el: web_sys::window()?
                .document()?
                .body()?
                .query_selector(select)
                .ok()? 
        } )
    }

}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("")
}

pub fn doc() -> web_sys::Document {
    window()
        .document()
        .expect("")
}

pub fn body() -> web_sys::HtmlElement {
    doc().body().expect("")
}

pub fn template() -> Result<(), JsValue> {
    let div = doc().create_element("div")?;
    let btns = doc().create_element("div")?;
    let head = doc().create_element("h1")?;
    let input = doc().create_element("input")?;
    let p = doc().create_element("p")?;
    let btn = doc().create_element("button")?;
    let btn2 = doc().create_element("button")?;
    head.set_inner_html(format!("Hello from Rust!").as_str());
    p.set_inner_html("this is wasm. This is coming from rust.");
    btn.set_inner_html("Hello");
    btn2.set_inner_html("Submit");
    body().append_child(&div)?;
    div.append_child(&head)?;
    div.append_child(&input)?;
    btns.append_child(&btn)?;
    btns.append_child(&btn2)?;
    div.append_child(&btns)?;
    div.append_child(&p)?;
    let canvas = doc()
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    div.append_child(&canvas)?;
    btn.set_inner_html("OK");
    canvas.set_width(640);
    canvas.set_height(480);
    body().append_child(&canvas)?;
    Ok(())
}

pub struct Doc {  }
pub struct Body { body: web_sys::HtmlElement, style: String }

pub mod web {

    use super::*;
    use web_sys::{MessageEvent, WebSocket};

    #[wasm_bindgen]
    pub fn new_ws() -> Result<(), JsValue> {
        let ws = WebSocket::new("wss://localhost:3009");
        let ws_clone = ws.clone();
        let msg_cb = Closure::wrap(Box::new(move |ev: MessageEvent| {
            if let Ok(abuf) = ev.data().dyn_into::<js_sys::ArrayBuffer>() {
                let barray = js_sys::Uint8Array::new(&abuf);

            } else if let Ok(blob) = ev.data().dyn_into::<web_sys::Blob>() {
                let fr = web_sys::FileReader::new().unwrap();
                fr.read_as_array_buffer(&blob).expect("");
            } else if let Ok(text) = ev.data().dyn_into::<js_sys::JsString>() {
                let fr = web_sys::FileReader::new().unwrap();
                let frc = fr.clone();
                let load = Closure::wrap(Box::new(move |_ev: web_sys::ProgressEvent| {
                    let barray = js_sys::Uint8Array::new(&frc.result().unwrap());
                }) as Box<dyn FnMut(web_sys::ProgressEvent)>);

            } else {

            }
        }) as Box<dyn FnMut(MessageEvent)>);
        msg_cb.forget();
        let err_cb = Closure::wrap(Box::new(move |_| {

        }) as Box<dyn FnMut(JsValue)>);
        err_cb.forget();
        let open_cb = Closure::wrap(Box::new(move |_| {

        }) as Box<dyn FnMut(JsValue)>);
        open_cb.forget();
        Ok(())
    }    
}

pub mod store {
    use super::*;
    use js_sys::JSON;

    pub struct Store {
        local: web_sys::Storage,
        data: ItemVec,
        name: String,
    }

    impl Store {
        pub fn new(name: &str) -> Option<Self> {
            let window = window();
            if let Ok(Some(local_storage)) = window.local_storage() {
                let mut store = Store {
                    local: local_storage,
                    name: name.to_string(),
                    data: ItemVec::new() 
                };
                store.get();
                Some(store)
            } else { None }
        }

        pub fn get(&mut self) -> Option<()> {
            Some(())
        }
    }

    pub struct Item { id: i32, data: String }
    pub struct ItemVec { items: Vec<Item> }

    impl ItemVec {
        pub fn new() -> Self {
            Self {
                items: Vec::new(),
            }
        }
    }

}

pub fn get_time(amt: f64) -> std::time::SystemTime {
    let secs = (amt as u64) / 1_000;
    let nsec = ((amt as u32) % 1_000) * 1_000_000;
    std::time::UNIX_EPOCH + std::time::Duration::new(secs, nsec)
}

pub mod macros {

    #[macro_export]
    macro_rules! clog {
        ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
    }
}

pub struct Canvas {
    pub canvas: web_sys::HtmlCanvasElement,
    pub context: web_sys::CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new() -> Option<Self> {
        if let Ok(canvas) = doc().create_element("canvas").unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>() {
                let cxt = canvas.get_context("2d")
                    .unwrap().unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();
                Some ( Self {
                    canvas, context: cxt
                } )
        } else {
            None
        }
    }

    pub fn begin(self) -> () { self.context.begin_path(); }

    pub fn add(&mut self, p1: f64, p2: f64, p3: f64, p4: f64) -> () {
        self.context.arc(p1, p2, p3, p4, f64::consts::PI).unwrap();
    }

    pub fn moveto(&mut self, p1: f64, p2: f64) -> () {
        self.context.move_to(p1, p2);
    }

    pub fn stroke(self) { self.context.stroke() }
}

pub fn test() {
    let f = Rc::new(RefCell::new(0));
    let fc = f.clone();
    let mut i = 0;
}

pub trait JsPromise {

    fn exec(&mut self) -> Promise;

}

//#[wasm_bindgen]
pub async fn get_users() -> Result<JsValue, JsValue> {
    let users = reqwest::get("http://localhost:3001/api/all").await.unwrap().text().await.unwrap();
    let ul = doc().create_element("ul").unwrap();
    for user in serde_json::to_vec(&users).unwrap() {
        let li = doc().create_element("li").unwrap();
        li.set_inner_html(user.to_string().as_str());
        ul.append_child(&li).unwrap();
    }
    body().append_child(&ul).unwrap();
    let val = JsValue::from_str(users.as_str());
    Ok(val)
}

pub mod element {
    use super::*;

    pub fn div() -> web_sys::HtmlElement {
        let div = doc().create_element("div").expect("");
        doc().append_child(&div).unwrap();
        div.dyn_into::<HtmlElement>().unwrap()
    }
}
