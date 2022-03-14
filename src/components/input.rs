use yew::{function_component, html, Callback, classes, InputEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct InputProps {
  pub id: String, 
  pub label: String,
  pub value: String,
  pub on_input: Callback<InputEvent>,
  pub class: String,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
  html! {
    <div class={classes!("w-full", props.class.clone())}>
      <label for={props.id.clone()} class="uppercase font-medium text-xs form-label inline-block mb-2 text-gray-700">{props.label.clone()}</label>
      <input id={props.id.clone()} name={props.id.clone()} class="form-control block w-full px-3 py-1.5 text-base font-normal text-gray-700 bg-white bg-clip-padding border border-solid border-gray-300 rounded transition ease-in-out m-0 focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none" value={props.value.clone()} oninput={&props.on_input} />
    </div>
  }
}
