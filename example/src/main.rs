use std::sync::{Arc, Mutex};

use afast::{AFast, AFastData, Error, handler, register};

#[derive(Debug, Clone, AFastData)]
enum Sex {
    Male { id: i64 },
    Female { name: String },
}

#[derive(Debug, Clone, AFastData)]
struct Request {
    sex: Sex,
    id: i64,
    name: String,
    #[validate(
        required("name is required"),
        min(1, "name must be at least 1 character long"),
        max(100, "name must be at most 10 characters long")
    )]
    age: u32,
    hobbies: Vec<Request2>,
    tags: Vec<String>,
    gender: Option<bool>,
}

#[derive(Debug, Clone, AFastData)]
struct Request2 {
    id: i64,
    name: String,
}

#[derive(Debug, AFastData)]
pub struct Response {
    sex: Sex,
    id: i64,
    name: String,
    age: u32,
    hobbies: Vec<Request2>,
    tags: Vec<String>,
    gender: Option<bool>,
}

#[handler(desc("Get user information"))]
async fn get_user(_state: Arc<Mutex<String>>, req: Request) -> Result<Response, Error> {
    Ok(Response {
        id: req.id,
        name: req.name.clone(),
        age: req.age,
        hobbies: req.hobbies.clone(),
        tags: req.tags.clone(),
        gender: req.gender,
        sex: req.sex.clone(),
    })
}

#[derive(Debug, AFastData)]
struct Req2 {
    id: i64,
}

#[derive(Debug, AFastData)]
struct Resp2 {
    id: i64,
    name: String,
}

#[handler(desc("Get user by id"))]
async fn get_id(_state: Arc<Mutex<String>>, req: Req2) -> Result<Resp2, Error> {
    Ok(Resp2 {
        id: req.id,
        name: "John".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new("".to_string()));

    let server = AFast::new(state, register! { get_user, get_id })
        .set_js(true) // Auto generate JS client
        .set_doc(true); // Auto generate documentation

    server.serve(&"127.0.0.1:8080", &"127.0.0.1:8081").await.unwrap();
}
