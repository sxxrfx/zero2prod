mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
}
