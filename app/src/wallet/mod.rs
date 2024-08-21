use leptos::{component, view, IntoView};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(module = "/src/wallet/wallet.js")]
extern "C" {
    fn listProviders();
}

#[component]
pub fn Wallet() -> impl IntoView {
    let on_click = |_| {
        listProviders();
    };

    view! {
        <h1>Wallet</h1>
        <button on:click=on_click>"Load Providers"</button>
        <div id="providerButtons" />
        <div id="walletAddress" />
    }
}
