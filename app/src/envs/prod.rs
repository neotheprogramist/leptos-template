use clap::Parser;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Error, Surreal,
};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub database: Surreal<Client>,
}
impl AppContext {
    pub async fn new(
        address: &str,
        username: &str,
        password: &str,
        namespace: &str,
        database: &str,
    ) -> Result<Self, Error> {
        let client = Surreal::new::<Ws>(address).await?;
        client.signin(Root { username, password }).await?;
        client.use_ns(namespace).use_db(database).await?;

        Ok(AppContext { database: client })
    }
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, env)]
    surrealdb_address: String,

    #[arg(long, env)]
    surrealdb_username: String,

    #[arg(long, env)]
    surrealdb_password: String,

    #[arg(long, env)]
    surrealdb_namespace: String,

    #[arg(long, env)]
    surrealdb_database: String,
}

pub async fn get_app_context() -> Result<AppContext, Error> {
    let args = Args::parse();

    AppContext::new(
        &args.surrealdb_address,
        &args.surrealdb_username,
        &args.surrealdb_password,
        &args.surrealdb_namespace,
        &args.surrealdb_database,
    )
    .await
}
