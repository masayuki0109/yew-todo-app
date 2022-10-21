use yew::{function_component, html, Callback};

use crate::types::TodosListProps;

#[function_component(TodosList)]
pub fn todos_list(TodosListProps { todos, on_click }: &TodosListProps) -> Html {
    todos
        .iter()
        .map(|todo| {
            let on_todo_select = {
                let on_click = on_click.clone();
                let todo = todo.clone();
                Callback::from(move |_| on_click.emit(todo.clone()))
            };

            html! {
                <>
                    <input type="checkbox" checked={todo.done} />
                    <p onclick={on_todo_select}> {
                        format!("{}: {}", todo.id, todo.title)
                    }
                    </p>
                </>
            }
        })
        .collect()
}
