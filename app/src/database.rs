use leptos::{
    component, create_signal, event_target, expect_context, server, spawn_local, view, IntoView,
    ServerFnError, SignalGet, SignalSet,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use web_sys::HtmlInputElement;

use crate::envs::AppContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueWithId {
    id: Thing,
    value: String,
}
#[derive(Debug, Clone, Serialize)]
struct Value {
    value: String,
}
#[server]
pub async fn update_data(value: String) -> Result<Vec<ValueWithId>, ServerFnError> {
    let context = expect_context::<AppContext>();
    let _: Vec<ValueWithId> = context
        .database
        .create("value")
        .content(Value { value })
        .await?;
    let all: Vec<ValueWithId> = context.database.select("value").await?;

    Ok(all)
}

#[component]
pub fn Database() -> impl IntoView {
    let (value, set_value) = create_signal("".to_string());
    let (display_value, set_display_value) = create_signal(vec![]);
    let on_click = move |_| {
        spawn_local(async move {
            let result = update_data(value.get()).await;
            set_display_value.set(result.unwrap());
        });
    };

    view! {
        <h1 class="text-2xl font-bold mb-4">"Welcome to Leptos!"</h1>
        <input
            class="p-2 border border-gray-300 rounded mb-4"
            type="text"
            on:input=move |e| { set_value.set(event_target::<HtmlInputElement>(&e).value()) }
        />
        <button
            class="bg-blue-500 text-white py-2 px-4 rounded hover:bg-blue-600 mb-4"
            on:click=on_click>
            "Add"
        </button>
        <div class="grid grid-cols-2 gap-4">
            {
                move || display_value.get().iter().cloned().map(|x| {
                    view! {
                        <div class="flex flex-col bg-gray-100 p-2 rounded">
                            <span class="font-semibold">{"ID: "}{x.id.to_string()}</span>
                            <span>{x.value}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }
        </div>
    }
}
