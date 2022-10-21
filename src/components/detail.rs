use yew::{function_component, html};

use crate::types::TodosDetailProps;

#[function_component(TodoDetail)]
pub fn todo_detail(TodosDetailProps { todo }: &TodosDetailProps) -> Html {

    html! {
        <div>
            <h3>{todo.title.clone()}</h3>
            <h3>{for todo.description.clone()}</h3>
        </div>
    }
}