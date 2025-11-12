use std::sync::{Arc, Mutex};

use afast::{AFast, AFastData, AFastKind, Error, Field, Kind, Tag, handler, register};

#[derive(Debug, Clone, AFastData, AFastKind)]
enum Sex {
    Male { id: i64 },
    Female { name: String },
}

#[derive(Debug, Clone, AFastData, AFastKind)]
struct Request {
    id: i64,
    name: String,
    #[validate(
        required("name is required"),
        min(1, "name must be at least 1 character long"),
        max(100, "name must be at most 10 characters long")
    )]
    age: u32,
    hobbies: Vec<Hobby>,
    tags: Vec<String>,
    gender: Option<bool>,
    sex: Sex,
}

#[derive(Debug, Clone, AFastData, AFastKind)]
struct Hobby {
    id: i64,
    name: String,
}

#[derive(Debug, AFastData, AFastKind)]
pub struct Response {
    sex: Sex,
    id: i64,
    name: String,
    age: u32,
    hobbies: Vec<Hobby>,
    tags: Vec<String>,
    gender: Option<bool>,
}

#[handler(desc("Get user information"), ns("api.user"))]
async fn get_user(
    _state: Arc<Mutex<String>>,
    _header: Header,
    req: Request,
) -> Result<Response, Error> {
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

async fn auth(_state: Arc<Mutex<String>>, header: Header) -> Result<(), Error> {
    println!("Token: {:?}", header);
    Ok(())
}

#[derive(Debug, AFastData, AFastKind)]
struct Req2 {
    id: i64,
}

#[derive(Debug, AFastData, AFastKind)]
struct Resp2 {
    id: i64,
    name: String,
}

#[handler(desc("Get user by id"), mw("auth"), ns("api"))]
async fn get_id(_state: Arc<Mutex<String>>, _header: Header, req: Req2) -> Result<Resp2, Error> {
    Ok(Resp2 {
        id: req.id,
        name: "John".to_string(),
    })
}

#[derive(Debug, Clone, AFastData, AFastKind)]
struct Header {
    id: u32,
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new("".to_string()));

    let server = AFast::<Mutex<String>, Header>::new(state).service(
        "user",
        "",
        register! { get_user, get_id },
    );

    server
        .serve(
            #[cfg(feature = "tcp")]
            &"127.0.0.1:8080",
            #[cfg(any(feature = "http", feature = "ws"))]
            &"127.0.0.1:8081",
        )
        .await
        .unwrap();
}
