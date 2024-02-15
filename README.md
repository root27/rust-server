# Rust Server

This is basic actix web server implementation using Rust. It includes CRUD operations using MongoDB.


## How to run

```bash

cargo run 

```

### API Endpoints

1. POST /create_user

```json
{
    "name": "",
    "email": "",
    "address": ""
}

```

2. GET /get_all


3. GET /get_user/{name}


4. POST /update_user/{id}

```json
{
    "email": "",
    "address": "",
    "name": ""
}

```

5. GET /delete/{id}







