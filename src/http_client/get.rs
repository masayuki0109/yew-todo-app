use gloo_net::http::Request;

use crate::types::Todo;

pub async fn todos() -> Vec<Todo> {
    Request::get("/todos")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
