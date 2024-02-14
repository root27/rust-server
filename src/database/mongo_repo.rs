
use std::str::FromStr;

use crate::models::User;
use crate::models::UpdateRequest;

use mongodb::results::DeleteResult;
use mongodb::{bson::{doc, extjson::de::Error}, results::{InsertOneResult, UpdateResult}, Client, Collection};
use mongodb::bson::oid::ObjectId;


use futures::stream::StreamExt;

pub struct MongoRepo {
    col: Collection<User>
}



impl MongoRepo {

    pub async fn init() -> Self {
        let client = Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
        let db = client.database("test");
        let col = db.collection("users");
        MongoRepo {
            col
        }
    }

    pub async fn create_user (&self, user: User) -> Result<InsertOneResult, Error> {

        let new_user = User {
            id: None,
            name: user.name,
            email: user.email,
            address: user.address
        };

       

        // Rest of the code...
        let result = self.col.insert_one(new_user, None).await
        .expect("Failed to insert document");

        Ok(result)

        }


    pub async fn get_user (&self, name:String) -> Result<User, Error> {

        let filter = doc! {"name": name};

        let user = self.col.find_one(filter, None).await
        .expect("Failed to get user");
    
        Ok(user.unwrap())



    }


    pub async fn get_all(&self) -> Result<Vec<User>, Error> {

        let mut cursor = self.col.find(None, None).await;


        let users = Vec::new();


        while let Some(doc) = cursor.next().await {

            users.push(doc)
            
        }

        


        Ok(users)

    }


    pub async fn update_user(&self, request: UpdateRequest, id: String) -> Result<UpdateResult, Error> {


        let object_id = ObjectId::from_str(&id);
        

        match object_id {

            Ok(object_id) => {
                
                let filter = doc!{"_id": object_id};

                let options = doc! {
                    "$set": {
                        "name": request.name,
                        "email": request.email,
                        "address": request.address
                    }
                };

                let result = self.col.update_one(filter,options,None).await
        .expect("Failed to update");


         Ok(result)


            }

            Err(e) => {
                Err(Error::from(e))
            }

        }
    
    }


    pub async fn delete_user(&self, id: String) -> Result<DeleteResult, Error> {

        let object_id = ObjectId::from_str(&id);

        match object_id {

            Ok(id) => {
                let filter = doc!{"_id": id};

                let result = self.col.delete_one(filter, None).await
                .expect("Failed to delete");

                Ok(result)


            }


            Err(e) => {
                Err(Error::from(e))
            }





        }



        


    }


}
    




