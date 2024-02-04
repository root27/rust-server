use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]

pub struct User {
    pub name: String,
    pub age: u8,
    pub address: String,
}

