mod components;
mod http_client;
mod types;

use crate::components::{detail::TodoDetail, form::InputFrom, list::TodosList};
use crate::http_client::get;
use gloo_net::http::Request;
use types::{NewTodo, Todo};
use wasm_bindgen::JsValue;
use web_sys::console::log_1;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let todos = use_state(std::vec::Vec::new);
    {
        let todos = todos.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_todos: Vec<Todo> = get::todos().await;

                    todos.set(fetched_todos);
                });
                || ()
            },
            (),
        );
    }

    let selected_todo = use_state(|| None);
    let on_todo_select = {
        let selected_todo = selected_todo.clone();
        Callback::from(move |todo: Todo| selected_todo.set(Some(todo)))
    };

    let detail = selected_todo.as_ref().map(|todo| {
        html! {
            <TodoDetail todo={todo.clone()} />
        }
    });

    let on_add = {
        let todos = todos.clone();
        Callback::from(move |title: String| {
            let data = NewTodo {
                title,
                description: None,
            };

            let data_serialized = serde_json::to_string_pretty(&data).unwrap();
            {
                let todos = todos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let post_request = Request::post("/todos")
                        .header("Content-Type", "application/json")
                        .body(wasm_bindgen::JsValue::from(&data_serialized))
                        .send()
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();
                    let deserialize_todo: Todo = serde_json::from_str(&post_request).unwrap();
                    let mut new_todos = (*todos).clone();
                    new_todos.push(deserialize_todo);
                    todos.set(new_todos);
                });
            }
        })
    };

    html! {
        <>
            <h1>{"My blog"}</h1>
            <form>
            <InputFrom {on_add} />
            </form>
            <div>
                <h3>{"todos list"}</h3>
                <TodosList todos={(*todos).clone()} on_click={on_todo_select.clone()} />
           </div>
           {for detail}
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
