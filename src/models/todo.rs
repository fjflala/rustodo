#[derive(Clone, PartialEq)]
pub struct Todo {
  pub id: usize,
  pub content: String,
  pub completed: bool,
}