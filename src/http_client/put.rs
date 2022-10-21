use gloo_net::http::Request;

pub async fn done_todo(done_serialized: &String, id: i32) -> String {
    Request::put(&format!("todos/{}", id))
        .header("Content-Type", "application/json")
        .body(wasm_bindgen::JsValue::from(done_serialized))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}

