use wasm_bindgen::prelude::*;
use super::*;
use web_sys::{Document, Element, HtmlElement, Window, console};
use js_sys::{Promise, Reflect, WebAssembly, Uint8ClampedArray};

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
