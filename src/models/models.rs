use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

#[derive(Deserialize,Serialize, Debug)]

pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id : Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub address: String,
}


#[derive(Serialize)]

pub struct Response {
    pub message: String
}


#[derive(Deserialize,Serialize)]

pub struct UpdateRequest {
    pub email: String,
    pub name: String,
    pub address: String
}
