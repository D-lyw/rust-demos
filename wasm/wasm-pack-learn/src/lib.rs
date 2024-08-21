mod utils;
mod web;

pub use web::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, my-wasm-pack-template!");
}

#[wasm_bindgen]
pub fn call_console_log() {
    log("Hello, my-wasm-pack-template! in console");
}

#[wasm_bindgen]
pub fn print_complex_pattern() {
    let pattern = r#"
    *****     *****
   *     *   *     *
  *       * *       *
 *         *         *
*        WASM        *
 *                   *
  *               *
   *     * *     *
    *****   *****
    "#;
    log(pattern);
}

#[wasm_bindgen]
pub async fn async_call() -> Result<JsValue, JsValue> {
    // 模拟一个异步操作
    let future = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::resolve(
        &JsValue::from_str("异步操作完成"),
    ));

    // 等待异步操作完成
    let result = future.await?;

    // 在控制台输出结果
    log(&format!("异步函数结果: {:?}", result));

    // 返回结果
    Ok(result)
}

#[wasm_bindgen]
pub fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }

    let mid = arr.len() / 2;
    let left = merge_sort(arr[..mid].to_vec());
    let right = merge_sort(arr[mid..].to_vec());

    merge(left, right)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();
    let mut left_peek = left_iter.next();
    let mut right_peek = right_iter.next();

    loop {
        match (left_peek, right_peek) {
            (Some(l), Some(r)) => {
                if l <= r {
                    result.push(l);
                    left_peek = left_iter.next();
                } else {
                    result.push(r);
                    right_peek = right_iter.next();
                }
            }
            (Some(l), None) => {
                result.push(l);
                result.extend(left_iter);
                break;
            }
            (None, Some(r)) => {
                result.push(r);
                result.extend(right_iter);
                break;
            }
            (None, None) => break,
        }
    }

    result
}

#[wasm_bindgen]
pub struct Person {
    name: String,
    age: u32,
}

#[wasm_bindgen]
impl Person {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    #[wasm_bindgen]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen]
    pub fn get_age(&self) -> u32 {
        self.age
    }

    #[wasm_bindgen]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[wasm_bindgen]
    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }

    #[wasm_bindgen]
    pub fn introduce(&self) -> String {
        format!("你好，我叫{}，今年{}岁。", self.name, self.age)
    }

    #[wasm_bindgen]
    pub fn have_birthday(&mut self) {
        self.age += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_new() {
        let person = Person::new("张三".to_string(), 25);
        assert_eq!(person.get_name(), "张三");
        assert_eq!(person.get_age(), 25);
    }

    #[test]
    fn test_person_set_name() {
        let mut person = Person::new("张三".to_string(), 25);
        person.set_name("李四".to_string());
        assert_eq!(person.get_name(), "李四");
    }

    #[test]
    fn test_person_set_age() {
        let mut person = Person::new("张三".to_string(), 25);
        person.set_age(30);
        assert_eq!(person.get_age(), 30);
    }

    #[test]
    fn test_person_introduce() {
        let person = Person::new("张三".to_string(), 25);
        assert_eq!(person.introduce(), "你好，我叫张三，今年25岁。");
    }

    #[test]
    fn test_person_have_birthday() {
        let mut person = Person::new("张三".to_string(), 25);
        person.have_birthday();
        assert_eq!(person.get_age(), 26);
    }
}
