use yew::{function_component, html};

use crate::presentation::routing::main_router::MainRouter;

#[function_component(App)]
pub fn app() -> Html {
    html! { <MainRouter/> }
}
