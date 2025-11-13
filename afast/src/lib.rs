//! # AFast
//!
//! **AFast** is a high-performance asynchronous Rust backend framework designed
//! to simplify building networked applications. It supports multiple protocols
//! via feature flags and provides automatic code generation for clients
//! (TypeScript and JavaScript), API documentation, and field validation.
//!
//! ## Instructions
//!
//! ### Supported Protocol Features
//!
//! You can enable the following features in your `Cargo.toml`:
//!
//! - `http` - enable HTTP support
//!   - `/` - Document path (feature flag `doc`)
//!   - `/api` - HTTP API endpoints
//!   - `/code/{service}/{lang}` - Client code (feature flag `js`|`ts` ...)
//!   - `/doc` - Service list (feature flag `doc`)
//!   - `/doc/{service}` - Handler defintions and documentation (feature flag `doc`)
//! - `ws` - enable WebSocket support
//!   - `/ws` - WebSocket endpoint
//! - `tcp` - enable TCP support
//! - `doc` - enable API documentation generation
//! - `js` - enable JavaScript client generation (auto enabled `code`)
//! - `ts` - enable TypeScript client generation (auto enabled `code`)
//! - `code` - enable code generation
//!
//! **Note on TCP usage:**  
//!
//! If the `tcp` feature is enabled, the `AFast::serve` method takes two arguments:
//!
//! 1. The TCP address to listen on (`"127.0.0.1:8080"`).  
//! 2. The HTTP/WS address (`"127.0.0.1:8081"`) for web clients and generated JS/TS clients.
//!
//! This allows you to run TCP and HTTP/WS servers simultaneously in the same application.
//!
//! ### Key Features
//!
//! - **`handler` Macro**: Declare HTTP endpoints with minimal boilerplate
//!   - Automatic TypeScript/JavaScript client generation
//!   - Namespace support for organized API structure (`ns("api.v1.user")`)
//!   - Middleware chaining for authentication/validation (`mws("auth")`)
//!   - Descriptive API documentation generation (`desc("Get user info")`)
//! - Automatic field validation with custom rules
//! - Async handler functions with state management
//! - Flexible multi-protocol support: HTTP, WS, TCP
//!
//! #### Handler Macro Overview
//!
//! The `#[handler]` attribute macro transforms async functions into full-featured API endpoints:
//!
//! ```rust
//! #[handler(desc("Get user information"), ns("api.v1.user"), mws("auth"))]
//! async fn get_user(state: Arc<Mutex<String>>, header: Header, req: Request) -> Result<Response, Error> {
//!     // Your business logic
//! }
//! ```
//!
//! **Macro Parameters:**
//!
//! - `desc("description")` - API description for documentation
//! - `ns("api.v1.user")` - Namespace for nested JS client generation
//! - `mws("auth,validation")` - Middleware chain for pre-processing
//!
//! **Generated Output:**
//!
//! - Type-safe HTTP endpoints
//! - Nested JavaScript client structure
//! - TypeScript type definitions  
//! - OpenAPI documentation
//!
//! ### Upcoming Features / Development Plan
//!
//! - Nested structure validation for complex types
//! - Enable or disable js / ts / document by feature flags
//! - Add command for generating client code
//! - Generate client code for additional languages: Java, Kotlin, C#, Rust, etc.
//! - Improved code generation templates for easier integration
//! - Enhanced error handling and validation reporting
//!
//! ## Example
//!
//! ```rust
//! use std::sync::{Arc, Mutex};
//!
//! use afast::{AFast, AFastData, AFastKind, Error, Field, Kind, Tag, handler, register};
//!
//! #[derive(Debug, Clone, AFastData, AFastKind)]
//! enum Sex {
//!     Other,
//!     Custom(#[validate(desc("Custom user sex 0"))] i32, String),
//!     Male {
//!         #[validate(desc("Male user id"))]
//!         id: i64,
//!     },
//!     Female {
//!         #[validate(desc("Female user name"))]
//!         name: String,
//!     },
//! }
//!
//! #[derive(Debug, Clone, AFastData, AFastKind)]
//! struct Request {
//!     #[validate(desc("User ID"))]
//!     id: i64,
//!     #[validate(desc("User name"))]
//!     name: String,
//!     #[validate(
//!         desc("User age"),
//!         required("age is required"),
//!         min(1, "age must be at least 1"),
//!         max(256, "age must be at most 256")
//!     )]
//!     age: u32,
//!     #[validate(desc("User hobbies"))]
//!     hobbies: Vec<Hobby>,
//!     #[validate(desc("User tags"))]
//!     tags: Vec<String>,
//!     #[validate(desc("User gender"))]
//!     gender: Option<bool>,
//!     #[validate(desc("User sex"))]
//!     sex: Sex,
//! }
//!
//! #[derive(Debug, Clone, AFastData, AFastKind)]
//! struct Hobby {
//!     id: i64,
//!     name: String,
//! }
//!
//! #[derive(Debug, AFastData, AFastKind)]
//! pub struct Response {
//!     sex: Sex,
//!     id: i64,
//!     name: String,
//!     age: u32,
//!     hobbies: Vec<Hobby>,
//!     tags: Vec<String>,
//!     gender: Option<bool>,
//! }
//!
//! #[handler(desc("Get user information"), ns("api.user"))]
//! async fn get_user(
//!     _state: Arc<Mutex<String>>,
//!     _header: Header,
//!     req: Request,
//! ) -> Result<Response, Error> {
//!     Ok(Response {
//!         id: req.id,
//!         name: req.name.clone(),
//!         age: req.age,
//!         hobbies: req.hobbies.clone(),
//!         tags: req.tags.clone(),
//!         gender: req.gender,
//!         sex: req.sex.clone(),
//!     })
//! }
//!
//! async fn auth(_state: Arc<Mutex<String>>, header: Header) -> Result<(), Error> {
//!     println!("Token: {:?}", header);
//!     Ok(())
//! }
//!
//! #[derive(Debug, AFastData, AFastKind)]
//! struct Req2 {
//!     id: i64,
//! }
//!
//! #[derive(Debug, AFastData, AFastKind)]
//! struct Resp2 {
//!     id: i64,
//!     name: String,
//! }
//!
//! #[handler(desc("Get user by id"), mw("auth"), ns("api"))]
//! async fn get_id(_state: Arc<Mutex<String>>, _header: Header, req: Req2) -> Result<Resp2, Error> {
//!     Ok(Resp2 {
//!         id: req.id,
//!         name: "John".to_string(),
//!     })
//! }
//!
//! #[derive(Debug, Clone, AFastData, AFastKind)]
//! struct Header {
//!     id: u32,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let state = Arc::new(Mutex::new("".to_string()));
//!
//!     let server = AFast::<Mutex<String>, Header>::new(state).service(
//!         "user",
//!         "User service",
//!         register! { get_user, get_id },
//!     );
//!
//!     server
//!         .serve(
//!             #[cfg(feature = "tcp")]
//!             &"127.0.0.1:8080",
//!             #[cfg(any(feature = "http", feature = "ws"))]
//!             &"127.0.0.1:8081",
//!         )
//!         .await
//!         .unwrap();
//! }
//! ```
//!

use std::sync::Arc;

pub use afast_macros::*;
use tokio::net::ToSocketAddrs;

#[cfg(feature = "doc")]
mod doc;
mod error;
#[cfg(feature = "js")]
mod js;
#[cfg(feature = "ts")]
mod ts;
pub use error::Error;

pub trait AFastData: Sized {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(buf: &[u8]) -> Result<(Self, usize), Error>;
    fn validate(&self) -> Result<(), Vec<&'static str>>;
}

/// The type of a handler function.
///
/// Each handler receives shared state `Arc<T>` and a binary request slice `&[u8]`,
/// and returns a Future that resolves to a binary response `Vec<u8>` or an `Error`.
pub type Handler<T, H> = dyn Fn(
        T,
        H,
        &[u8],
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<u8>, Error>> + Send + 'static>,
    > + Send
    + Sync;

pub type Middleware<T, H> = dyn Fn(
        T,
        H,
    )
        -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Error>> + Send + 'static>>
    + Send
    + Sync;

/// Generic handler struct that wraps a user-defined handler function.
///
/// Contains a handler ID, name, JS request code, TS request code, and the processing function.
pub struct HandlerGeneric<T, H>
where
    T: Clone + Send + Sync + 'static,
    H: AFastKind + AFastData,
{
    /// Unique handler ID
    pub id: u32,
    /// Handler name
    pub name: &'static str,

    /// JS client
    #[cfg(feature = "js")]
    pub js: String,

    /// TS client
    #[cfg(feature = "ts")]
    pub ts: String,

    /// API documentation
    #[cfg(feature = "doc")]
    pub doc: String,

    /// Middleware functions
    pub middleware: Box<Middleware<T, H>>,
    /// Namespace for this handler
    pub namespace: Vec<String>,
    /// Actual processing function
    pub func: Box<Handler<T, H>>,
}

/// Core AFast service struct.
///
/// Maintains shared state and registered handlers.
pub struct AFast<T, H>
where
    T: Clone + Send + Sync + 'static,
    H: AFastKind + AFastData + Clone + Send + Sync + 'static,
{
    /// Shared application state
    state: T,
    /// Registered handler list
    handlers: Arc<Vec<HandlerGeneric<T, H>>>,

    /// Client code
    #[cfg(feature = "code")]
    codes: std::collections::HashMap<String, String>,

    services: std::collections::HashSet<(String, String, usize)>,

    /// Includs JS util
    #[cfg(feature = "js")]
    js_util: bool,

    /// Includs TS util
    #[cfg(feature = "ts")]
    ts_util: bool,
}

impl<T, H> AFast<T, H>
where
    T: Clone + Send + Sync + 'static,
    H: AFastKind + AFastData + Clone + Send + Sync + 'static,
{
    /// Create a new AFast service instance
    ///
    /// # Arguments
    /// * `state` - Shared application state
    /// * `handlers` - List of registered handlers
    ///
    /// # Returns
    /// A new AFast instance
    pub fn new(state: T) -> Self {
        Self {
            state,
            handlers: Arc::new(vec![]),
            #[cfg(feature = "code")]
            codes: std::collections::HashMap::new(),
            services: std::collections::HashSet::new(),
            #[cfg(feature = "js")]
            js_util: true,
            #[cfg(feature = "ts")]
            ts_util: true,
        }
    }

    #[cfg(feature = "js")]
    /// Enable or disable JS util
    /// The service will take effect after this
    pub fn disable_js_util(mut self) -> Self {
        self.js_util = false;
        self
    }

    #[cfg(feature = "ts")]
    /// Enable or disable TS util
    /// The service will take effect after this
    pub fn disable_ts_util(mut self) -> Self {
        self.ts_util = false;
        self
    }

    /// Register a service with additional handlers
    pub fn service(mut self, name: &str, desc: &str, handlers: Vec<HandlerGeneric<T, H>>) -> Self {
        if handlers.is_empty() {
            println!("No handlers found for service {}", name);
            return self;
        }

        if self.services.iter().any(|s| s.0 == name) {
            panic!("Service {} already registered", name);
        }

        self.services
            .insert((name.to_string(), desc.to_string(), handlers.len()));

        #[cfg(feature = "js")]
        self.codes.insert(
            format!("code/{}/js", name),
            js::gen_js_code(self.js_util, self.handlers.len(), &handlers),
        );

        #[cfg(feature = "ts")]
        self.codes.insert(
            format!("code/{}/ts", name),
            ts::gen_ts_code(self.js_util, self.handlers.len(), &handlers),
        );

        #[cfg(feature = "doc")]
        self.codes.insert(
            format!("doc/{}", name),
            doc::gen_doc(self.handlers.len(), &handlers),
        );

        let existing_handlers = Arc::get_mut(&mut self.handlers).unwrap();
        existing_handlers.extend(handlers);
        self.handlers = Arc::clone(&self.handlers);

        self
    }

    /// Start the server
    ///
    /// Depending on feature flags, this will start TCP, HTTP, and/or WebSocket services.
    ///
    /// # Arguments
    /// * `addr_tcp` - TCP listening address (only active if `tcp` feature enabled)
    /// * `addr_http` - HTTP/WebSocket listening address (only active if `http` or `ws` feature enabled)
    ///
    /// # Returns
    /// Result<(), Error>
    pub async fn serve<A: ToSocketAddrs>(
        &self,
        #[cfg(feature = "tcp")] addr_tcp: A,
        #[cfg(any(feature = "http", feature = "ws"))] addr_http: A,
    ) -> Result<(), Error> {
        #[cfg(feature = "tcp")]
        {
            // Clone shared state and handler list for TCP processing
            let state = Arc::clone(&self.state);
            let handlers = Arc::clone(&self.handlers);

            // Bind TCP listener
            let listener = tokio::net::TcpListener::bind(addr_tcp).await.unwrap();

            #[cfg(any(feature = "http", feature = "ws"))]
            {
                // Spawn TCP listener task if HTTP/WS is also enabled
                tokio::spawn(async move {
                    loop {
                        let (mut socket, _) = listener.accept().await.unwrap();

                        // Read 4-byte length prefix
                        let mut body = [0u8; 4];
                        tokio::io::AsyncReadExt::read_exact(&mut socket, &mut body)
                            .await
                            .unwrap();
                        let len = u32::from_be_bytes(body) as usize;

                        // Read message body
                        let mut body = vec![0u8; len];
                        tokio::io::AsyncReadExt::read_exact(&mut socket, &mut body)
                            .await
                            .unwrap();

                        let (header, size) = match H::from_bytes(&body) {
                            Ok(h) => h,
                            Err(_) => {
                                continue;
                            }
                        };

                        // Parse sequence number and handler ID
                        let seq: usize = u32::from_be_bytes([
                            body[size + 0],
                            body[size + 1],
                            body[size + 2],
                            body[size + 3],
                        ]) as usize;
                        let id: usize = u32::from_be_bytes([
                            body[size + 4],
                            body[size + 5],
                            body[size + 6],
                            body[size + 7],
                        ]) as usize;

                        // Call handler
                        let handler = &handlers[id];

                        let fut = (handler.middleware)(state.clone(), header.clone());
                        match fut.await {
                            Ok(_) => {}
                            Err(_) => {
                                continue;
                            }
                        }
                        let fut = (handler.func)(state.clone(), header, &body[size + 8..]);
                        let res = fut.await.unwrap();

                        // Build response: len + seq + id + response
                        let mut final_res = Vec::with_capacity(12 + res.len());
                        final_res.extend_from_slice(&(len + 8).to_be_bytes());
                        final_res.extend_from_slice(&seq.to_be_bytes());
                        final_res.extend_from_slice(&id.to_be_bytes());
                        final_res.extend_from_slice(&res);

                        tokio::io::AsyncWriteExt::write_all(&mut socket, &final_res)
                            .await
                            .unwrap();
                    }
                });
            }

            #[cfg(not(any(feature = "http", feature = "ws")))]
            {
                // Blocking TCP loop if HTTP/WS is disabled
                loop {
                    let (mut socket, _) = listener.accept().await.unwrap();

                    let mut body = [0u8; 4];
                    tokio::io::AsyncReadExt::read_exact(&mut socket, &mut body)
                        .await
                        .unwrap();
                    let len = u32::from_be_bytes(body) as usize;

                    let mut body = vec![0u8; len];
                    tokio::io::AsyncReadExt::read_exact(&mut socket, &mut body)
                        .await
                        .unwrap();

                    let (header, size) = match H::from_bytes(&body) {
                        Ok(h) => h,
                        Err(_) => {
                            continue;
                        }
                    };

                    // Parse sequence number and handler ID
                    let seq: usize = u32::from_be_bytes([
                        body[size + 0],
                        body[size + 1],
                        body[size + 2],
                        body[size + 3],
                    ]) as usize;
                    let id: usize = u32::from_be_bytes([
                        body[size + 4],
                        body[size + 5],
                        body[size + 6],
                        body[size + 7],
                    ]) as usize;

                    // Call handler
                    let handler = &handlers[id];

                    let fut = (handler.middleware)(state.clone(), header.clone());
                    match fut.await {
                        Ok(_) => {}
                        Err(_) => {
                            continue;
                        }
                    }
                    let fut = (handler.func)(state.clone(), header, &body[size + 8..]);
                    let res = fut.await.unwrap();

                    let mut final_res = Vec::with_capacity(12 + res.len());
                    final_res.extend_from_slice(&(len + 8).to_be_bytes());
                    final_res.extend_from_slice(&seq.to_be_bytes());
                    final_res.extend_from_slice(&id.to_be_bytes());
                    final_res.extend_from_slice(&res);

                    tokio::io::AsyncWriteExt::write_all(&mut socket, &final_res)
                        .await
                        .unwrap();
                }
            }
        }

        #[cfg(any(feature = "http", feature = "ws"))]
        {
            let state = self.state.clone();
            let handlers = Arc::clone(&self.handlers);

            // TCP listener for HTTP/WS
            let listener = tokio::net::TcpListener::bind(addr_http).await.unwrap();

            let app = axum::Router::new();

            // WebSocket route /ws
            #[cfg(feature = "ws")]
            let app = app.route(
                "/ws",
                axum::routing::any(
                    move |axum::Extension((state, handlers)): axum::Extension<(
                        T,
                        Arc<Vec<HandlerGeneric<T, H>>>,
                    )>,
                          ws: axum::extract::ws::WebSocketUpgrade| async move {
                        ws.on_upgrade(|mut ws| async move {
                            loop {
                                let data = ws.recv().await.unwrap().unwrap();
                                if let axum::extract::ws::Message::Binary(body) = data {
                                    if body.len() < 8 {
                                        continue;
                                    }

                                    let (header, size) = match H::from_bytes(&body) {
                                        Ok(h) => h,
                                        Err(_) => {
                                            continue;
                                        }
                                    };

                                    let seq: usize = u32::from_be_bytes([
                                        body[size + 0],
                                        body[size + 1],
                                        body[size + 2],
                                        body[size + 3],
                                    ])
                                        as usize;
                                    let id: usize = u32::from_be_bytes([
                                        body[size + 4],
                                        body[size + 5],
                                        body[size + 6],
                                        body[size + 7],
                                    ]) as usize;

                                    let handler = &handlers[id];

                                    let fut =
                                        (handler.middleware)(state.clone(), header.clone());
                                    match fut.await {
                                        Ok(_) => {}
                                        Err(_) => {
                                            continue;
                                        }
                                    }
                                    let fut = (handler.func)(
                                        state.clone(),
                                        header,
                                        &body[size + 8..],
                                    );
                                    let res = fut.await.unwrap();
                                    let mut final_res = Vec::with_capacity(8 + res.len());
                                    final_res.extend_from_slice(&seq.to_be_bytes());
                                    final_res.extend_from_slice(&id.to_be_bytes());
                                    final_res.extend_from_slice(&res);
                                    ws.send(axum::extract::ws::Message::Binary(final_res.into()))
                                        .await
                                        .unwrap();
                                }
                            }
                        })
                    },
                ),
            );

            // HTTP route /api
            #[cfg(feature = "http")]
            let app = app.route(
                "/api",
                axum::routing::post(
                    move |axum::Extension((state, handlers)): axum::Extension<(
                        T,
                        Arc<Vec<HandlerGeneric<T, H>>>,
                    )>,
                          body: axum::body::Bytes| async move {
                        let (header, size) = match H::from_bytes(&body[..]) {
                            Ok(h) => h,
                            Err(e) => {
                                return axum::response::Response::builder()
                                    .status(400)
                                    .body(http_body_util::Full::new(axum::body::Bytes::from(
                                        e.to_string(),
                                    )))
                                    .unwrap();
                            }
                        };

                        if body.len() < size + 4 {
                            return axum::response::Response::builder()
                                .status(400)
                                .body(http_body_util::Full::new(axum::body::Bytes::from(
                                    "Invalid request",
                                )))
                                .unwrap();
                        }

                        let id = u32::from_be_bytes([
                            body[size + 0],
                            body[size + 1],
                            body[size + 2],
                            body[size + 3],
                        ]);
                        let handler = &handlers[id as usize];

                        let fut = (handler.middleware)(state.clone(), header.clone());
                        match fut.await {
                            Ok(_) => {}
                            Err(e) => {
                                return axum::response::Response::builder()
                                    .status(400)
                                    .body(http_body_util::Full::new(axum::body::Bytes::from(
                                        e.to_string(),
                                    )))
                                    .unwrap();
                            }
                        }
                        let fut = (handler.func)(state.clone(), header, &body[size + 4..]);
                        match fut.await {
                            Ok(res) => {
                                let mut final_res = Vec::with_capacity(4 + res.len());
                                final_res.extend_from_slice(&id.to_be_bytes());
                                final_res.extend_from_slice(&res);
                                axum::response::Response::builder()
                                    .status(200)
                                    .body(http_body_util::Full::new(axum::body::Bytes::from(
                                        final_res,
                                    )))
                                    .unwrap()
                            }
                            Err(e) => axum::response::Response::builder()
                                .status(400)
                                .body(http_body_util::Full::new(axum::body::Bytes::from(
                                    e.to_string(),
                                )))
                                .unwrap(),
                        }
                    },
                ),
            );

            #[cfg(feature = "code")]
            let app = app.route(
                "/code/{service}/{lang}",
                axum::routing::get(
                    move |axum::Extension(codes): axum::Extension<
                        std::collections::HashMap<String, String>,
                    >,
                          axum::extract::Path((service, lang)): axum::extract::Path<(
                        String,
                        String,
                    )>| async move {
                        let code =
                            if let Some(code) = codes.get(&format!("code/{}/{}", service, lang)) {
                                code.to_string()
                            } else {
                                return axum::response::Response::builder()
                                    .status(404)
                                    .body(http_body_util::Full::new(axum::body::Bytes::from(
                                        "404 Not Found",
                                    )))
                                    .unwrap();
                            };
                        axum::response::Response::builder()
                            .status(200)
                            .header(
                                axum::http::header::CONTENT_TYPE,
                                "text/javascript; charset=utf-8",
                            )
                            .body(http_body_util::Full::new(axum::body::Bytes::from(code)))
                            .unwrap()
                    },
                ),
            );

            #[cfg(feature = "doc")]
            let app = app.route(
                "/doc",
                axum::routing::get(
                    move |axum::Extension(services): axum::Extension<
                        std::collections::HashSet<(String, String, usize)>,
                    >| async move {
                        let code = format!(
                            r#"{{"services":[{}]}}"#,
                            services
                                .iter()
                                .map(|(n, d, c)| format!(
                                    r#"{{"name":"{}","desc":"{}","count":{}}}"#,
                                    n, d, c
                                ))
                                .collect::<Vec<String>>()
                                .join(",")
                        );
                        axum::response::Response::builder()
                            .status(200)
                            .header(
                                axum::http::header::CONTENT_TYPE,
                                "application/json; charset=utf-8",
                            )
                            .body(http_body_util::Full::new(axum::body::Bytes::from(code)))
                            .unwrap()
                    },
                ),
            );

            #[cfg(feature = "doc")]
            let app = app.route(
                "/doc/{service}",
                axum::routing::get(
                    move |axum::Extension(codes): axum::Extension<
                        std::collections::HashMap<String, String>,
                    >,
                          axum::extract::Path(service): axum::extract::Path<
                        String
                    >| async move {
                        let code =
                            if let Some(code) = codes.get(&format!("doc/{}", service)) {
                                code.to_string()
                            } else {
                                return axum::response::Response::builder()
                                    .status(404)
                                    .body(http_body_util::Full::new(axum::body::Bytes::from(
                                        "404 Not Found",
                                    )))
                                    .unwrap();
                            };
                        axum::response::Response::builder()
                            .status(200)
                            .header(
                                axum::http::header::CONTENT_TYPE,
                                "application/json; charset=utf-8",
                            )
                            .body(http_body_util::Full::new(axum::body::Bytes::from(code)))
                            .unwrap()
                    },
                ),
            );

            #[cfg(feature = "doc")]
            static DIST_DIR: include_dir::Dir =
                include_dir::include_dir!("$CARGO_MANIFEST_DIR/dist");
            #[cfg(feature = "doc")]
            let app = app.fallback(|uri: axum::http::Uri| async move {
                let path = uri.path().trim_start_matches('/').trim();
                let path = if path.is_empty() { "index.html" } else { path };

                let file = DIST_DIR
                    .get_file(path)
                    .or_else(|| DIST_DIR.get_file("index.html"));

                if let Some(file) = file {
                    let mime = mime_guess::from_path(path).first_or_octet_stream();
                    axum::response::Response::builder()
                        .status(axum::http::StatusCode::OK)
                        .header("Content-Type", mime.as_ref())
                        .body(http_body_util::Full::from(file.contents()))
                        .unwrap()
                } else {
                    axum::response::Response::builder()
                        .status(axum::http::StatusCode::NOT_FOUND)
                        .body(http_body_util::Full::from("404 Not Found"))
                        .unwrap()
                }
            });

            // Attach shared state and CORS
            let app = app.layer(axum::Extension((state, handlers)));
            #[cfg(feature = "code")]
            let app = app.layer(axum::Extension(self.codes.clone()));
            let app = app.layer(axum::Extension(self.services.clone()));
            let app = app.layer(
                tower_http::cors::CorsLayer::new()
                    .allow_origin(tower_http::cors::AllowOrigin::any())
                    .allow_methods(tower_http::cors::AllowMethods::any())
                    .allow_headers(tower_http::cors::AllowHeaders::any()),
            );

            axum::serve(listener, app).await.unwrap();

            Ok(())
        }
    }
}

pub trait AFastKind: Sized {
    fn kind() -> Kind;

    fn field(name: &str) -> Field;
}

#[derive(Debug)]
pub struct Tag {
    pub name: Option<String>,
    pub description: Option<String>,
    pub required: Option<String>,
    pub min: Option<(i64, String)>,
    pub max: Option<(i64, String)>,
}

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub kind: Kind,
    pub tag: Option<Tag>,
}

#[derive(Debug)]
pub enum Kind {
    Unit,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Bool,
    String,
    Vec(Box<Kind>),
    Enum { variants: Vec<Kind> },
    Struct { fields: Vec<Field> },
    Tuple(Vec<Kind>),
    Nullable(Box<Kind>),
}
