use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Properties, FocusEvent, InputEvent, TargetCast};
use crate::components::{button::Button, input::Input};

#[derive(Properties, PartialEq)]
pub struct FormProps {
  #[prop_or_default]
  pub inside: String,
  pub on_submit: Callback<String>
}

#[function_component(Form)]
pub fn button(props: &FormProps) -> Html {
  let on_submit = props.on_submit.clone();
  let value = use_state(|| "".to_string());
  let on_input = {
    let value = value.clone();
    Callback::from(move |e: InputEvent| {
      let input: HtmlInputElement = e.target_unchecked_into();
      value.set(input.value());
    })
  };

  let onsubmit: Callback<FocusEvent> = {
    let value = value.clone();
    let on_submit = on_submit.clone();
    Callback::from(move |e: FocusEvent| {
      e.prevent_default();
      on_submit.emit(String::from(value.to_string()));
      value.set("".to_string());
    })
  };

  let is_disabled = if value.clone().len() == 0 { true } else { false };

  html! {
    <form {onsubmit}>
      <Input class="mb-2" label="Content" value={(*value).clone()} {on_input} />
      <Button disabled={is_disabled}>{"Add todo"}</Button>
    </form>    
  }
}
