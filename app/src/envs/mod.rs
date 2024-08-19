#[cfg(not(feature = "prod"))]
pub mod dev;
#[cfg(feature = "prod")]
pub mod prod;

#[cfg(not(feature = "prod"))]
pub use dev::{get_app_context, AppContext};
#[cfg(feature = "prod")]
pub use prod::{get_app_context, AppContext};
