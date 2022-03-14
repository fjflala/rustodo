use yew::{function_component, html, classes, Children, Properties};

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
  #[prop_or_default]
  pub children: Children,
  pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
  let default_classes = "w-full inline-block px-6 py-2.5 font-medium text-xs leading-tight uppercase rounded shadow-md";
  let mut class = classes!(default_classes.clone());
  
  if props.disabled {
    class.push("bg-gray-200 text-gray-400 cursor-not-allowed");
  } else {
    class.push("bg-blue-600 text-white hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out")
  }

  html! {<button disabled={props.disabled} {class}>{props.children.clone()}</button>}
}
