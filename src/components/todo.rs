use wasm_bindgen::JsValue;
use web_sys::{console::log_1, InputEvent, HtmlInputElement};
use yew::{function_component, html, Callback, TargetCast};

use crate::types::TodoProps;

#[function_component(Todo)]
pub fn todos(TodoProps { todo, on_click }: &TodoProps) -> Html {
    let onclick = {
        let on_click = on_click.clone();
        let todo = todo.clone();
        Callback::from(move |_| on_click.emit(todo.clone()))
    };

    let oninput = {
        Callback::from(move |e: InputEvent| {
          log_1(&JsValue::from(e.target_unchecked_into::<HtmlInputElement>().checked()))
        })
    };

    html! {
        <li>
            <input type="checkbox" checked={todo.done} oninput={oninput} />
            {
                format!("{}: {}", todo.id, todo.title)
            }
            <button {onclick}>{"削除"}</button>
        </li>
    }
}
