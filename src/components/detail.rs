use yew::{function_component, html};

use crate::types::PostsDetailProps;

#[function_component(PostDetail)]
pub fn post_detail(PostsDetailProps { post }: &PostsDetailProps) -> Html {
    html! {
        <div>
            <h3>{post.title.clone()}</h3>
            <h3>{post.body.clone()}</h3>
        </div>
    }
}