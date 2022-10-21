use crate::types;
use web_sys::{ HtmlInputElement, InputEvent};
use yew::{function_component, html, Callback, TargetCast};

use crate::types::TodoProps;

#[function_component(Todo)]
pub fn todos(
    TodoProps {
        todo,
        on_click,
        on_change_value,
    }: &TodoProps,
) -> Html {
    let onclick = {
        let on_click = on_click.clone();
        let todo = todo.clone();
        Callback::from(move |_| on_click.emit(todo.clone()))
    };

    let oninput = {
        let on_change_value = on_change_value.clone();
        let todo = todo.clone();
        Callback::from(move |e: InputEvent| {
            let done = e.target_unchecked_into::<HtmlInputElement>().checked();
            on_change_value.emit(types::Todo {
                done,
                ..todo.clone()
            });
        })
    };

    html! {
        <li class="list-group-item d-flex justify-content-between align-items-start">
            <div>
                <input class="form-check-input me-2" type="checkbox" checked={todo.done} oninput={oninput} />
                {
                    format!(" {}", todo.title)
                }
            </div>
            <button {onclick} class="btn btn-dark">{"削除"}</button>
        </li>
    }
}
