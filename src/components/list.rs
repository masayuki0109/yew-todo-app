use yew::{function_component, html, Callback};

use crate::types::PostsListProps;

#[function_component(PostsList)]
pub fn posts_list(PostsListProps { posts, on_click }: &PostsListProps) -> Html {
    posts
        .iter()
        .map(|post| {
            let on_post_select = {
                let on_click = on_click.clone();
                let post = post.clone();
                Callback::from(move |_| on_click.emit(post.clone()))
            };

            html! {
                <p onclick={on_post_select}> {
                    format!("{}: {}", post.id, post.title)
                }
                </p>
            }
        })
        .collect()
}
