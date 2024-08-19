use surrealdb::{engine::local::Db, Error, Surreal};

#[derive(Debug, Clone)]
pub struct AppContext {
    pub database: Surreal<Db>,
}
impl AppContext {
    pub async fn new() -> Result<Self, Error> {
        Ok(AppContext {
            database: Surreal::new(()).await?,
        })
    }
}

pub async fn get_app_context() -> Result<AppContext, Error> {
    AppContext::new().await
}
