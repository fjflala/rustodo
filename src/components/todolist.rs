use yew::{function_component, html, Callback, Html, Properties};
use crate::models::{todo::Todo};
use crate::components::{todoitem::TodoItem};

#[derive(Properties, PartialEq)]
pub struct TodoListProps {
  pub todos: Vec<Todo>,
  pub on_complete: Callback<Todo>
}

#[function_component(TodoList)]
pub fn todo_list(TodoListProps { todos, on_complete }: &TodoListProps) -> Html {
  let items = todos.iter().map(|todo| html! {
    <TodoItem todo={todo.clone()} {on_complete} />
  }).collect::<Html>();
  html! {<ul class="-mx-4 mb-4">{items}</ul>}
}
