
use yew::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Clone, PartialEq)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub published: bool
}

#[derive(Deserialize, Serialize)]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
}



#[derive(Properties, PartialEq)]
pub struct TodosListProps {
    pub todos: Vec<Todo>,
    pub on_click: Callback<Todo>,
    pub on_change_value: Callback<Todo>
}

#[derive(Properties, PartialEq)]
pub struct TodoProps {
    pub todo: Todo,
    pub on_click: Callback<Todo>,
    pub on_change_value: Callback<Todo>
}

#[derive(Deserialize,Serialize)]
pub struct TodoDoneRequest {
    pub done: bool,
}


#[derive(Clone, Properties, PartialEq)]
pub struct TodosDetailProps {
    pub todo: Todo,
}