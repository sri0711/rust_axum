use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: ObjectId,
    pub name: String,
    pub age: i8,
    pub email: Option<String>,
    pub phone: String,
}
