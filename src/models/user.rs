// the output to our `create_user` handler
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}