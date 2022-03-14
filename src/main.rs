mod components;
mod models;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::{form::Form, todolist::TodoList, switch::Switch};
use crate::models::{todo::Todo};

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

enum Msg {
  AddTodo(String),
  MarkAsComplete(Todo),
  FilterCompleted(bool),
}

struct Model {
  todos: Vec<Todo>,
  filter_completed: bool,
}

impl Component for Model {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    let todos: Vec<Todo> = vec![
      Todo {
        id: 0,
        content: String::from("Types"),
        completed: false
      },
      Todo {
        id: 1,
        content: String::from("Vic"),
        completed: true
      }
    ];

    Self {
      todos: todos,
      filter_completed: false,
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::FilterCompleted(checked) => {
        self.filter_completed = checked;
        true
      }
      Msg::MarkAsComplete(todo) => {
        let pos = self.todos.iter().position(|x| x.id == todo.id).unwrap();
        self.todos[pos].completed = !self.todos[pos].completed;
        true
      }
      Msg::AddTodo(content) => {
        let len = self.todos.len();

        let new_todo = Todo {
          id: len,
          content: content,
          completed: false,
        };

        self.todos.push(new_todo);

        true
      }
    }
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let on_submit = ctx.link().callback(|content: String| Msg::AddTodo(content));
    let on_complete = ctx.link().callback(|todo: Todo| Msg::MarkAsComplete(todo));
    let completed_todos = self.todos.iter().filter(|todo| todo.completed == true).collect::<Vec<&Todo>>();
    let uncompleted_todos = self.todos.clone().into_iter().filter(|todo| todo.completed == false).collect::<Vec<Todo>>();
    let on_toggle = ctx.link().callback(|checked: bool| Msg::FilterCompleted(checked));

    let get_todo_list = move || -> Html {
      if self.filter_completed {
        html! {
          <TodoList todos={uncompleted_todos.clone()} {on_complete} />
        }
      } else {
        html! {
          <TodoList todos={self.todos.clone()} {on_complete} />
        }
      }
    };

    html! {
      <div class="bg-gray-100 h-screen w-screen flex items-center justify-center">
        <div class="w-2/5">
          <div class="w-full bg-white rounded-md p-4 drop-shadow-sm">
            <h1 class="text-xl mb-4 font-semibold uppercase text-gray-600">{"Rustodo"}</h1>
            <div class="-mx-4 py-2 px-4 border-b-2 border-b-gray-100 text-xs text-gray-400 uppercase font-medium flex items-end">
              <div class="w-full">
                <span>{"Completed "}</span>
                {completed_todos.len()}
                <span>{" out of "}</span>
                {self.todos.len()}
              </div>
              <Switch {on_toggle} />
            </div>
            {get_todo_list()}
            <Form {on_submit}/>
          </div>
          <div class="text-center mt-2 text-gray-400 text-xs">
            <p>
              {"Made with ♥ by Francisco"} 
            </p>
            <p class="flex justify-center">
              <a target="_blank" href="" class="mr-2">{"Repository"}</a>
              <a target="_blank" href="https://yew.rs/" class="mr-2">{"Yew"}</a>
              <a target="_blank" href="https://tailwindcss.com/">{"Tailwind"}</a>
            </p>
          </div>
        </div>
      </div>
    }
  }
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::start_app::<Model>();
}