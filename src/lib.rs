pub use action_api_derive::Action;

pub trait Action {
    fn icon(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn label(&self) -> &'static str;
}