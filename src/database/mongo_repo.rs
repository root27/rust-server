
use crate::models::User;
use mongodb::{bson::{doc, extjson::de::Error},Client, Collection, results::InsertOneResult};


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
            age: user.age,
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
}
    




