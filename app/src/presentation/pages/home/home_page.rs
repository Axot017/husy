use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback};

#[function_component(HomePage)]
pub fn app() -> Html {
    let onclick = Callback::once(move |_| {
        spawn_local(async {

        });
    });
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "Go subpage" }</button>
        </div>
    }
}
