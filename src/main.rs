mod components;
mod types;

use crate::components::{detail::PostDetail, list::PostsList};
use gloo_net::http::Request;
use types::{Post};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let posts = use_state(std::vec::Vec::new);
    {
        let posts = posts.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_posts: Vec<Post> = Request::get("/posts")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    posts.set(fetched_posts);
                });
                || ()
            },
            (),
        );
    }

    let selected_post = use_state(|| None);
    let on_post_select = {
        let selected_post = selected_post.clone();
        Callback::from(move |post: Post| selected_post.set(Some(post)))
    };

    let detail = selected_post.as_ref().map(|post| {
        html! {
            <PostDetail post={post.clone()} />
        }
    });
    html! {
        <>
            <h1>{"My blog"}</h1>
            <form>
                <input /><submit/>
            </form>
            <div>
                <h3>{"posts list"}</h3>
                <PostsList posts={(*posts).clone()} on_click={on_post_select.clone()} />
           </div>
           {for detail}
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
