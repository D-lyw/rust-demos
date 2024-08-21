use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Element, Window};

#[wasm_bindgen]
pub fn add_element_to_body() -> Result<(), JsValue> {
    // 获取窗口对象
    let window: Window = web_sys::window().expect("没有全局 `window` 对象");
    
    // 获取文档对象
    let document: Document = window.document().expect("应该有一个 `document` 在 `window` 上");
    
    // 创建一个新的 div 元素
    let div: Element = document.create_element("div")?;
    
    // 设置 div 的文本内容
    div.set_text_content(Some(format!("这是通过 WebAssembly 添加的新元素！插入时间：{}", js_sys::Date::new_0().to_string()).as_str()));
    
    // 将新创建的 div 添加到 body 中
    let body = document.body().expect("文档应该有一个 body");
    body.append_child(&div)?;
    
    Ok(())
}

#[wasm_bindgen]
pub async fn fetch_data() -> Result<JsValue, JsValue> {
    let window = web_sys::window().expect("没有全局 `window` 对象");
    let resp_value = JsFuture::from(window.fetch_with_str("https://jsonplaceholder.typicode.com/todos/1")).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;
    Ok(json)
}