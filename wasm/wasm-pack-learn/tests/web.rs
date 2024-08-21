//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use wasm_pack_learn::fetch_data;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_fetch_data() {
    // 模拟 fetch 函数
    let mock_fetch = js_sys::Function::new_with_args(
        "url",
        "return Promise.resolve(new Response(JSON.stringify({id: 1, title: 'test todo'}), {headers: {'Content-Type': 'application/json'}}));"
    );

    // 替换全局的 fetch 函数
    js_sys::Reflect::set(&js_sys::global(), &"fetch".into(), &mock_fetch).unwrap();

    // 调用 fetch_data 函数
    let result = fetch_data().await.unwrap();

    // 验证结果
    let obj = js_sys::Object::from(result);
    assert_eq!(
        js_sys::Reflect::get(&obj, &"id".into()).unwrap(),
        JsValue::from(1)
    );
    assert_eq!(
        js_sys::Reflect::get(&obj, &"title".into()).unwrap(),
        JsValue::from("test todo")
    );
}
