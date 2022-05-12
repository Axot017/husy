use wasm_bindgen_futures::spawn_local;
use yew::{function_component, html, Callback};

use crate::{
    core::di::get::GetIt, data::wallet::near_wallet_repository::NearWalletRepository,
    use_cases::wallet::initialize_wallet_use_case::InitializeWalletUseCase,
};

#[function_component(HomePage)]
pub fn app() -> Html {
    let aaa: InitializeWalletUseCase<NearWalletRepository> = GetIt::get();
    let onclick = Callback::once(move |_| {
        spawn_local(async move {
            let _result = &aaa.call().await;
        });
    });
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "Go subpage" }</button>
        </div>
    }
}
