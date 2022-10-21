use gloo_net::http::Request;

use crate::types::Post;

pub async fn posts() -> Vec<Post> {
    Request::get("/posts")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
