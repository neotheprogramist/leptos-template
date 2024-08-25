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

    let counter_stream: surrealdb::method::Stream<'_, Db, Vec<Counter>> = context
        .database
        .select("counter")
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

    spawn_local(async move {
        log!("spawn local");
        let mut stream = watch_data().await.unwrap().into_inner();
        while let Some(Ok(x)) = stream.next().await {
            log!("stream: {x}");
            set_count.set(x);
        }
    });

    view! {
        <h1>"My Dynamic Data"</h1>
        {move || count.get()}
    }
}
