use yew::{function_component, html, Html};
use yew_router::{BrowserRouter, Routable, Switch};

use crate::presentation::pages::{home::home_page::HomePage, subpage::Subpage};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/subpage")]
    Subpage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch_main(route: &MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<HomePage/>},
        MainRoute::Subpage => html! {<Subpage/>},
        MainRoute::NotFound => html! {<h1>{"Not found"}</h1>},
    }
}

#[function_component(MainRouter)]
pub fn main_router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<MainRoute> render={Switch::render(switch_main)} />
        </BrowserRouter>
    }
}
