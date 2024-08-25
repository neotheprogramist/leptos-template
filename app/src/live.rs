use futures::{stream, StreamExt};
use leptos::{
    component, create_signal, expect_context,
    logging::log,
    server,
    server_fn::codec::{StreamingText, TextStream},
    spawn_local, view, IntoView, ServerFnError, SignalGet, SignalSet,
};
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;

use crate::envs::AppContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Counter {
    pub counter: i64,
}

#[server]
pub async fn update_data() -> Result<(), ServerFnError> {
    let context = expect_context::<AppContext>();
    let db = context.database;

    let x: Vec<Counter> = db.select("counter").await?;
    if x.is_empty() {
        let _: Vec<Counter> = db.create("counter").content(Counter { counter: 0 }).await?;
    } else {
        let _: Vec<Counter> = db
            .update("counter")
            .content(Counter {
                counter: x.first().unwrap().counter + 1,
            })
            .await?;
    }

    Ok(())
}

#[server(output = StreamingText)]
pub async fn watch_data() -> Result<TextStream, ServerFnError> {
    let context = expect_context::<AppContext>();

    let counter_stream: surrealdb::method::Stream<Db, Vec<Counter>> = context
        .database
        .select("counter")
        .into_owned()
        .live()
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    let result = stream::unfold(counter_stream, |mut counter_stream| async move {
        if let Some(Ok(x)) = counter_stream.next().await {
            Some((Ok(x.data.counter.to_string()), counter_stream))
        } else {
            None
        }
    });

    Ok(TextStream::new(result))
}

#[component]
pub fn Live() -> impl IntoView {
    let (count, set_count) = create_signal(String::new());

    // Start the stream to watch data changes
    spawn_local(async move {
        log!("Starting data stream...");
        let mut stream = watch_data().await.unwrap().into_inner();
        while let Some(Ok(x)) = stream.next().await {
            log!("Stream received: {x}");
            set_count.set(x);
        }
    });

    // Function to handle button click and update data
    let on_click = move |_| {
        spawn_local(async move {
            log!("Button clicked - updating data...");
            if let Err(e) = update_data().await {
                log!("Error updating data: {e}");
            }
        });
    };

    view! {
        <div class="p-4 bg-gray-100 rounded shadow-md">
            <h1 class="text-2xl font-bold mb-4">"My Dynamic Data"</h1>
            <p class="text-lg">{move || count.get()}</p>
            <button
                class="mt-4 px-4 py-2 bg-blue-500 text-white font-semibold rounded hover:bg-blue-600"
                on:click=on_click
            >
                "Update Counter"
            </button>
        </div>
    }
}
