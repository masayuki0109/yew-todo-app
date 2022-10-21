use yew::{function_component, html, Callback};
use crate::components::todo::Todo;

use crate::types::TodosListProps;

#[function_component(TodosList)]
pub fn todos_list(TodosListProps { todos, on_click }: &TodosListProps) -> Html {
    todos
        .iter()
        .map(|todo| {
            let onclick = {
                let on_click = on_click.clone();
                let todo = todo.clone();
                Callback::from(move |_| on_click.emit(todo.clone()))
            };

            html! {
                <Todo todo={(*todo).clone()} on_click={onclick} />
            }
        })
        .collect()
}
