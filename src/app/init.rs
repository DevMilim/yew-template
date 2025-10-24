use yew::prelude::*;

pub struct InitPage;

#[derive(Properties, PartialEq)]
pub struct InitPageProps;

pub enum InitPageMsg {}

impl Component for InitPage {
    type Message = InitPageMsg;
    type Properties = InitPageProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {}
    }
}
