use yew::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Properties, PartialEq)]
pub struct PostsListProps {
    pub posts: Vec<Post>,
    pub on_click: Callback<Post>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct PostsDetailProps {
    pub post: Post,
}