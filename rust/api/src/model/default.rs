use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    Email: String,
    Password: String,
    Admin: bool,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Poll {
    Status: String,
    pub Questions: HashMap<String, i32>,
}
