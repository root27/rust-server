use serde::{Serialize, Deserialize};


#[derive(Deserialize)]

pub struct User {
    pub name: String,
    pub age: u8,
    pub address: String,
}


#[derive(Serialize)]

pub struct Response {
    pub message: String
}

