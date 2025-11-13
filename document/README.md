# AFast

**AFast** is a high-performance asynchronous Rust backend framework designed
to simplify building networked applications. It supports multiple protocols
via feature flags and provides automatic code generation for clients
(TypeScript and JavaScript), API documentation, and field validation.

## Instructions

### Supported Protocol Features

You can enable the following features in your `Cargo.toml`:

- `http` - enable HTTP support
  - `/` - Document path (feature flag `doc`)
  - `/api` - HTTP API endpoints
  - `/code/{service}/{lang}` - Client code (feature flag `js`|`ts` ...)
  - `/doc` - Service list (feature flag `doc`)
  - `/doc/{service}` - Handler defintions and documentation (feature flag `doc`)
- `ws` - enable WebSocket support
  - `/ws` - WebSocket endpoint
- `tcp` - enable TCP support
- `doc` - enable API documentation generation
- `js` - enable JavaScript client generation (auto enabled `code`)
- `ts` - enable TypeScript client generation (auto enabled `code`)
- `code` - enable code generation

**Note on TCP usage:**  

If the `tcp` feature is enabled, the `AFast::serve` method takes two arguments:

1. The TCP address to listen on (`"127.0.0.1:8080"`).  
2. The HTTP/WS address (`"127.0.0.1:8081"`) for web clients and generated JS/TS clients.

This allows you to run TCP and HTTP/WS servers simultaneously in the same application.

### Key Features

- **`handler` Macro**: Declare HTTP endpoints with minimal boilerplate
  - Automatic TypeScript/JavaScript client generation
  - Namespace support for organized API structure (`ns("api.v1.user")`)
  - Middleware chaining for authentication/validation (`mws("auth")`)
  - Descriptive API documentation generation (`desc("Get user info")`)
- Automatic field validation with custom rules
- Async handler functions with state management
- Flexible multi-protocol support: HTTP, WS, TCP

#### Handler Macro Overview

The `#[handler]` attribute macro transforms async functions into full-featured API endpoints:

```rust
#[handler(desc("Get user information"), ns("api.v1.user"), mws("auth"))]
async fn get_user(state: String, header: Header, req: Request) -> Result<Response, Error> {
    // Your business logic
}
```

**Macro Parameters:**

- `desc("description")` - API description for documentation
- `ns("api.v1.user")` - Namespace for nested JS client generation
- `mws("auth,validation")` - Middleware chain for pre-processing

**Generated Output:**

- Type-safe HTTP endpoints
- Nested JavaScript client structure
- TypeScript type definitions  
- OpenAPI documentation

### Upcoming Features / Development Plan

- Nested structure validation for complex types
- Enable or disable js / ts / document by feature flags
- Add command for generating client code
- Generate client code for additional languages: Java, Kotlin, C#, Rust, etc.
- Improved code generation templates for easier integration
- Enhanced error handling and validation reporting

## Example

```rust
use afast::{AFast, AFastData, AFastKind, Error, Field, Kind, Tag, handler, register};

#[derive(Debug, Clone, AFastData, AFastKind)]
enum Sex {
    Other,
    Custom(#[validate(desc("Custom user sex 0"))] i32, String),
    Male {
        #[validate(desc("Male user id"))]
        id: i64,
    },
    Female {
        #[validate(desc("Female user name"))]
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
    gender: Option<bool>,
    #[validate(desc("User sex"))]
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
    _state: String,
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

async fn auth(_state: String, header: Header) -> Result<(), Error> {
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
async fn get_id(_state: String, _header: Header, req: Req2) -> Result<Resp2, Error> {
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
    let state = "".to_string();

    let server = AFast::<String, Header>::new(state).service(
        "user",
        "User service",
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
```
