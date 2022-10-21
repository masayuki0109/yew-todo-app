use yew::{function_component, html, Callback};

use crate::types::TodoProps;

#[function_component(Todo)]
pub fn todos(TodoProps { todo, on_click }: &TodoProps) -> Html {
    let onclick = {
        let on_click = on_click.clone();
        let todo = todo.clone();
        Callback::from(move |_| on_click.emit(todo.clone()))
    };
    html! {
        <>
            <input type="checkbox" checked={todo.done} />
            <p> {
                format!("{}: {}", todo.id, todo.title)
            }
            </p>
            <button {onclick}>{"削除"}</button>
        </>
    }
}
