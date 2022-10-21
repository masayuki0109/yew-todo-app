use gloo_net::http::Request;

pub async fn todos(todo_serialized: &String) -> String {
    Request::post("/todos")
        .header("Content-Type", "application/json")
        .body(wasm_bindgen::JsValue::from(todo_serialized))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
