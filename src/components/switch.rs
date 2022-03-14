use wasm_bindgen::prelude::*;
use yew::{function_component, functional::*, html, classes,Event,  Callback, Properties};

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[derive(Properties, PartialEq)]
pub struct SwitchProps {
  pub on_toggle: Callback<bool>,
}

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
  let checked = use_state(|| false);
  let default_classes = "transition-all	appearance-none w-9 rounded-full h-5 align-top bg-white bg-no-repeat bg-contain focus:outline-none cursor-pointer shadow-sm";
  let mut class = classes!(default_classes.clone());

  if *checked {
    class.push("bg-blue-500")
  } else {
    class.push("bg-gray-300")
  }
  
  let get_styles = {
    let checked = checked.clone();
    let style_checked = "background-image: url('data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%22-4 -4 8 8%22%3E%3Ccircle r=%223%22 fill=%22%23fff%22/%3E%3C/svg%3E'); background-position: 100%;";
    let style_unchecked = "background-image: url('data:image/svg+xml,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%22-4 -4 8 8%22%3E%3Ccircle r=%223%22 fill=%22%23fff%22/%3E%3C/svg%3E'); background-position: 0;";
    move || -> String {
      if *checked {
        String::from(style_checked)
      } else {
        String::from(style_unchecked)
      }
    }
  };

  let onchange = {
    let checked = checked.clone();
    let on_toggle = props.on_toggle.clone();
    Callback::from(move |_: Event| {
      let new_value = !*checked;
      checked.set(new_value);
      on_toggle.emit(new_value);
    })
  };

  html! {
    <div class="w-full flex flex-col items-end justify-end">
      <label class="block text-gray-800" for="flexSwitchCheckDefault">{"Filter completed"}</label>
      <input {onchange} style={get_styles()} {class} type="checkbox" role="switch" id="flexSwitchCheckDefault" />
    </div>
  }
}
