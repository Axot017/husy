use yew::{function_component, html, Callback};
use yew_router::{history::History, hooks::use_history};

use crate::presentation::routing::main_router::MainRoute;

#[function_component(HomePage)]
pub fn app() -> Html {
    let history = use_history().unwrap();
    let onclick = Callback::once(move |_| history.push(MainRoute::Subpage));
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "Go subpage" }</button>
        </div>
    }
}
