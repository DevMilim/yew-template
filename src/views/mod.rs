use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::init::InitPage;

pub mod init;

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Init,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Init => html! { <InitPage /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="flex min-h-screen flex-col">
                <Switch<AppRoute> render={switch} />
            </div>
        </BrowserRouter>
    }
}
