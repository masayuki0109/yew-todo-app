use yew::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub published: bool
}

#[derive(Properties, PartialEq)]
pub struct TodosListProps {
    pub todos: Vec<Todo>,
    pub on_click: Callback<Todo>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct TodosDetailProps {
    pub todo: Todo,
}