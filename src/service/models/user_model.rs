use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: Option<String>,
    name: String,
    age: i8,
    email: Option<String>,
    phone: String,
}
