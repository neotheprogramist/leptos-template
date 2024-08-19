use std::io;

use app::{envs::get_app_context, App};
use axum::Router;
use fileserv::file_and_error_handler;
use leptos::{get_configuration, leptos_config::errors::LeptosConfigError, provide_context};
use leptos_axum::{generate_route_list, LeptosRoutes};
use shutdown::shutdown_signal;
use thiserror::Error;

pub mod fileserv;
mod shutdown;

#[derive(Debug, Error)]
enum ServerError {
    #[error(transparent)]
    LeptosConfig(#[from] LeptosConfigError),

    #[error(transparent)]
    Surrealdb(#[from] surrealdb::Error),

    #[error(transparent)]
    Io(#[from] io::Error),
}

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await?;
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_context = get_app_context().await?;

    // build our application with a route
    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || provide_context(app_context.clone()),
            App,
        )
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
