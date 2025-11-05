# AFast

**AFast** is a high-performance asynchronous Rust web framework designed
to simplify building networked applications. It supports multiple protocols
via feature flags and provides automatic code generation for clients
(TypeScript and JavaScript), API documentation, and field validation.

## Instrutions

### Supported Protocol Features

You can enable the following features in your `Cargo.toml`:

- `http` - enable HTTP support
  - `/api` - HTTP API endpoint
  - `/js` - JavaScript client
  - `/ts` - TypeScript client
- `ws` - enable WebSocket support
  - `/ws` - WebSocket endpoint
- `tcp` - enable TCP support

**Note on TCP usage:**  

If the `tcp` feature is enabled, the `AFast::serve` method takes two arguments:

1. The TCP address to listen on (`"127.0.0.1:8080"`).  
2. The HTTP/WS address (`"127.0.0.1:8081"`) for web clients and generated JS/TS clients.

This allows you to run TCP and HTTP/WS servers simultaneously in the same application.

### Key Features

- Automatic generation of TypeScript/JavaScript clients for your API
- Automatic generation of documentation
- Automatic field validation, including custom rules
- Async handler functions with state management
- Flexible multi-protocol support: HTTP, WS, TCP

### Upcoming Features / Development Plan

- Nested structure validation for complex types
- Enable or disable js / ts / document by feautre flags
- Add command for generating client code
- Generate client code for additional languages: Java, Kotlin, C#, Rust, etc.
- Improved code generation templates for easier integration
- Enhanced error handling and validation reporting

## Example

```rust
use std::sync::{Arc, Mutex};

use afast::{AFast, AFastData, Error, handler, register};

#[derive(Debug, Clone, AFastData)]
enum Sex {
    Male { id: i64 },
    Female { name: String },
}

#[derive(Debug, Clone, AFastData)]
struct Request {
    id: i64,
    name: String,
    #[validate(
        required("age is required"),
        min(1, "age must be greater than or equal to 1"),
        max(100, "age must be less than or equal to 100")
    )]
    age: u32,
    hobbies: Vec<Hobby>,
    tags: Vec<String>,
    is_active: Option<bool>,
    sex: Sex,
}

#[derive(Debug, Clone, AFastData)]
struct Hobby {
    id: i64,
    name: String,
}

#[derive(Debug, AFastData)]
pub struct Response {
    sex: Sex,
    id: i64,
    name: String,
    age: u32,
    hobbies: Vec<Hobby>,
    tags: Vec<String>,
    is_active: Option<bool>,
}

#[handler(desc("Get user information"))]
async fn get_user(_state: Arc<Mutex<String>>, req: Request) -> Result<Response, Error> {
    Ok(Response {
        id: req.id,
        name: req.name.clone(),
        age: req.age,
        hobbies: req.hobbies.clone(),
        tags: req.tags.clone(),
        is_active: req.is_active,
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

    let server = AFast::new(state, register! { get_user, get_id });

    server.serve(&"127.0.0.1:8080").await.unwrap();

    // Alternatively, you can start the server with TCP and HTTP/WS support:
    // first argument is TCP listening address, second argument is HTTP/WS listening address
    // server.serve(&"127.0.0.1:8080", &"127.0.0.1:8081").await.unwrap();
}
```
