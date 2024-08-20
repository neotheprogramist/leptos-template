use clap::Parser;
use surrealdb::{engine::local::Db, Error, Surreal};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub database: Surreal<Db>,
}
impl AppContext {
    pub async fn new(namespace: &str, database: &str) -> Result<Self, Error> {
        let client = Surreal::new(()).await?;
        client.use_ns(namespace).use_db(database).await?;

        Ok(AppContext { database: client })
    }
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, env, default_value = "dev")]
    surrealdb_namespace: String,

    #[arg(long, env, default_value = "dev")]
    surrealdb_database: String,
}

pub async fn get_app_context() -> Result<AppContext, Error> {
    let args = Args::parse();

    AppContext::new(&args.surrealdb_namespace, &args.surrealdb_database).await
}
