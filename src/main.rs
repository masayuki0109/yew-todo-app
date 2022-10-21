mod components;
mod http_client;
mod types;

use crate::components::{detail::TodoDetail, form::InputFrom, list::TodosList};
use crate::http_client::{delete, get, post, put};
use gloo_net::http::Request;
use types::{NewTodo, Todo, TodoDoneRequest};
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

    let on_click = {
        let todos = todos.clone();
        Callback::from(move |todo: Todo| {
            let todos = todos.clone();
            wasm_bindgen_futures::spawn_local(async move {
                delete::todo(todo.id).await;

                let new_todos = (*todos)
                    .clone()
                    .into_iter()
                    .filter(|t| t.id != todo.id)
                    .collect::<Vec<Todo>>();
                todos.set(new_todos);
            })
        })
    };

    let on_add = {
        let todos = todos.clone();
        Callback::from(move |title: String| {
            let input_todo = NewTodo {
                title,
                description: None,
            };

            let todo_serialized = serde_json::to_string_pretty(&input_todo).unwrap();
            {
                let todos = todos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let result = post::todos(&todo_serialized).await;
                    let deserialize_todo: Todo = serde_json::from_str(&result).unwrap();
                    let mut new_todos = (*todos).clone();
                    new_todos.push(deserialize_todo);
                    todos.set(new_todos);
                });
            }
        })
    };

    let on_change_value = {
        Callback::from(move |todo: Todo| {
            let done_serialized =
                serde_json::to_string_pretty(&TodoDoneRequest { done: todo.done }).unwrap();
            wasm_bindgen_futures::spawn_local(async move {
                put::done_todo(&done_serialized, todo.id).await;
            });
        })
    };

    html! {
        <>
            <h1>{"Todo App"}</h1>
            <form>
            <InputFrom {on_add} />
            </form>
            <div>
                <h3>{"todos list"}</h3>
                <TodosList todos={(*todos).clone()} on_click={on_click.clone()} on_change_value={on_change_value.clone()} />
           </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
