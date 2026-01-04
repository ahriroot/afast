use afast::{AFast, AFastData, AFastKind, Error, handler, middleware, register};

#[derive(Debug, Clone, AFastData, AFastKind)]
enum Sex {
    Male,
    Female,
    Other(#[validate(desc("Other sex id"))] i64),
    Custom {
        #[validate(desc("Custom sex name"))]
        name: String,
    },
}

#[derive(Debug, Clone, AFastData, AFastKind)]
struct Request {
    #[validate(desc("User ID"))]
    id: i64,
    #[validate(desc("User name"))]
    name: String,
    #[validate(
        desc("User age"),
        required("age is required"),
        min(1, "age must be at least 1"),
        max(256, "age must be at most 256")
    )]
    age: u32,
    #[validate(desc("User hobbies"))]
    hobbies: Vec<Hobby>,
    #[validate(desc("User tags"))]
    tags: Vec<String>,
    #[validate(desc("User gender"))]
    superuser: Option<bool>,
    #[validate(desc("User sex"))]
    sex: Sex,
    #[validate(desc("User number"))]
    number: f64,
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
    superuser: Option<bool>,
}

#[handler(desc("Get user information"), ns("api.user"))]
async fn get_user(_state: String, _header: Header, req: Request) -> Result<Response, Error> {
    Ok(Response {
        id: req.id,
        name: req.name.clone(),
        age: req.age,
        hobbies: req.hobbies.clone(),
        tags: req.tags.clone(),
        superuser: req.superuser,
        sex: req.sex.clone(),
    })
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

#[handler(desc("Get user by id"), ns("api"))]
async fn get_id(_state: String, _header: Header, req: Req2) -> Result<Resp2, Error> {
    Ok(Resp2 {
        id: req.id,
        name: "John".to_string(),
    })
}

#[derive(Debug, Clone, AFastData, AFastKind)]
struct Header {
    token: String,
}

#[middleware]
async fn auth(_state: String, header: Header) -> Result<Header, Error> {
    println!("Token: {:?}", header);
    Ok(header)
}

// async fn ws(
//     _state: String,
//     _header: Header,
//     mut receiver: tokio::sync::mpsc::Receiver<Vec<u8>>,
//     sender: tokio::sync::mpsc::Sender<Vec<u8>>,
// ) -> Result<(), Error> {
//     // 循环处理 receiver 中的消息，并通过 sender 发送响应
//     while let Some(msg) = receiver.recv().await {
//         let (req, _) = Req2::from_bytes(&msg).unwrap();
//         let resp = Resp2 {
//             id: req.id,
//             name: "John".to_string(),
//         };
//         sender.send(resp.to_bytes()).await.unwrap();
//     }

//     Ok(())
// }

#[tokio::main]
async fn main() {
    let state = "".to_string();

    let server = AFast::<String, Header>::new(state)
        .service("user", "User service", register! { get_user, get_id })
        .middleware(auth);

    server
        .serve(
            #[cfg(feature = "tcp")]
            &"127.0.0.1:8080",
            #[cfg(any(feature = "http", feature = "ws", feature = "doc", feature = "js", feature = "ts"))]
            &"127.0.0.1:8081",
        )
        .await
        .unwrap();
}
