use gloo_net::http::Request;

pub async fn todo(id: i32) {
    Request::delete(&format!("/todos/{}", id))
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
}
