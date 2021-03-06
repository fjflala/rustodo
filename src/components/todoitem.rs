use yew::{function_component, html, Callback, MouseEvent, Properties};
use crate::models::{todo::Todo};

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
  pub todo: Todo,
  pub on_complete: Callback<Todo>,
}

#[function_component(TodoItem)]
pub fn todo_list(props: &TodoItemProps) -> Html {
  let get_label = move || -> &str {
    if props.todo.completed {
      "Completed"
    } else {
      "Todo"
    }
  };

  let onclick = {
    let on_complete = props.on_complete.clone();
    let todo = props.todo.clone();
    Callback::from(move |_:MouseEvent| {
      on_complete.emit(todo.clone());
    })
  };
  

  html! {
    <li class="py-2 px-4 border-b-2 border-b-gray-100 flex justify-between items-center">
      <span class="text-gray-500 mr-2">
        {props.todo.content.clone()}
        <small class="text-gray-400 ml-2">{"(#"}{props.todo.id}{")"}</small>
      </span>
      <div class="flex items-center">
        <label for={props.todo.id.to_string()} class="uppercase text-xs text-gray-400 flex items-center">
          <span>{get_label()}</span>
          <input {onclick} id={props.todo.id.to_string()} name={props.todo.id.to_string()} class="ml-2 form-check-input appearance-none h-4 w-4 border border-gray-300 rounded-sm bg-white checked:bg-blue-600 checked:border-blue-600 focus:outline-none transition duration-200 bg-no-repeat bg-center bg-contain cursor-pointer" type="checkbox" checked={props.todo.completed} />
        </label>
      </div>
    </li>
  }
}
