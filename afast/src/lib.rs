//! # AFast
//! 
//! **AFast** is a high-performance asynchronous Rust web framework designed
//! to simplify building networked applications. It supports multiple protocols
//! via feature flags and provides automatic code generation for clients
//! (TypeScript and JavaScript), API documentation, and field validation.
//! 
//! ## Instrutions
//! 
//! ### Supported Protocol Features
//! 
//! You can enable the following features in your `Cargo.toml`:
//! 
//! - `http` - enable HTTP support
//!   - `/api` - HTTP API endpoint
//!   - `/js` - JavaScript client
//!   - `/ts` - TypeScript client
//! - `ws` - enable WebSocket support
//!   - `/ws` - WebSocket endpoint
//! - `tcp` - enable TCP support
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
//! - Automatic generation of TypeScript/JavaScript clients for your API
//! - Automatic generation of documentation
//! - Automatic field validation, including custom rules
//! - Async handler functions with state management
//! - Flexible multi-protocol support: HTTP, WS, TCP
//! 
//! ### Upcoming Features / Development Plan
//! 
//! - Nested structure validation for complex types
//! - Enable or disable js / ts / document by feautre flags
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
//! use afast::{AFast, AFastData, Error, handler, register};
//! 
//! #[derive(Debug, Clone, AFastData)]
//! enum Sex {
//!     Male { id: i64 },
//!     Female { name: String },
//! }
//! 
//! #[derive(Debug, Clone, AFastData)]
//! struct Request {
//!     id: i64,
//!     name: String,
//!     #[validate(
//!         required("age is required"),
//!         min(1, "age must be greater than or equal to 1"),
//!         max(100, "age must be less than or equal to 100")
//!     )]
//!     age: u32,
//!     hobbies: Vec<Hobby>,
//!     tags: Vec<String>,
//!     is_active: Option<bool>,
//!     sex: Sex,
//! }
//! 
//! #[derive(Debug, Clone, AFastData)]
//! struct Hobby {
//!     id: i64,
//!     name: String,
//! }
//! 
//! #[derive(Debug, AFastData)]
//! pub struct Response {
//!     sex: Sex,
//!     id: i64,
//!     name: String,
//!     age: u32,
//!     hobbies: Vec<Hobby>,
//!     tags: Vec<String>,
//!     is_active: Option<bool>,
//! }
//! 
//! #[handler(desc("Get user information"))]
//! async fn get_user(_state: Arc<Mutex<String>>, req: Request) -> Result<Response, Error> {
//!     Ok(Response {
//!         id: req.id,
//!         name: req.name.clone(),
//!         age: req.age,
//!         hobbies: req.hobbies.clone(),
//!         tags: req.tags.clone(),
//!         is_active: req.is_active,
//!         sex: req.sex.clone(),
//!     })
//! }
//! 
//! #[derive(Debug, AFastData)]
//! struct Req2 {
//!     id: i64,
//! }
//! 
//! #[derive(Debug, AFastData)]
//! struct Resp2 {
//!     id: i64,
//!     name: String,
//! }
//! 
//! #[handler(desc("Get user by id"))]
//! async fn get_id(_state: Arc<Mutex<String>>, req: Req2) -> Result<Resp2, Error> {
//!     Ok(Resp2 {
//!         id: req.id,
//!         name: "John".to_string(),
//!     })
//! }
//! 
//! #[tokio::main]
//! async fn main() {
//!     let state = Arc::new(Mutex::new("".to_string()));
//! 
//!     let server = AFast::new(state, register! { get_user, get_id });
//! 
//!     server.serve(&"127.0.0.1:8080").await.unwrap();
//! 
//!     // Alternatively, you can start the server with TCP and HTTP/WS support:
//!     // first argument is TCP listening address, second argument is HTTP/WS listening address
//!     // server.serve(&"127.0.0.1:8080", &"127.0.0.1:8081").await.unwrap();
//! }
//! ```
//! 

use std::sync::Arc;

pub use afast_macros::*;
use tokio::net::ToSocketAddrs;

mod error;
mod js;
pub use error::Error;

pub trait AFastData: Sized {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(buf: &[u8]) -> Result<(Self, usize), Error>;
    fn to_js(prefix: &str) -> String;
    fn to_ts(prefix: &str) -> String;
    fn to_js_type(prefix: &str) -> String;
    fn to_js_validate(prefix: &str) -> String;
    fn from_js(prefix: &str) -> String;
    fn validate(&self) -> Result<(), Vec<&'static str>>;
}

/// The type of a handler function.
///
/// Each handler receives shared state `Arc<T>` and a binary request slice `&[u8]`,
/// and returns a Future that resolves to a binary response `Vec<u8>` or an `Error`.
pub type Handler<T> = dyn Fn(
        Arc<T>,
        &[u8],
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Vec<u8>, Error>> + Send + 'static>,
    > + Send
    + Sync;

/// Generic handler struct that wraps a user-defined handler function.
///
/// Contains a handler ID, name, JS request code, TS request code, and the processing function.
pub struct HandlerGeneric<T>
where
    T: Send + Sync + 'static,
{
    /// Unique handler ID
    pub id: u32,
    /// Handler name
    pub name: &'static str,
    /// JS client
    pub js: String,
    /// TS client
    pub ts: String,
    /// Actual processing function
    pub func: Box<Handler<T>>,
}

/// Core AFast service struct.
///
/// Maintains shared state and registered handlers.
pub struct AFast<T>
where
    T: Send + Sync + 'static,
{
    /// Shared application state
    state: Arc<T>,
    /// Registered handler list
    handlers: Arc<Vec<HandlerGeneric<T>>>,
    /// Flag to output JS client
    js: bool,
    /// Flag to output documentation
    doc: bool,
}

impl<T> AFast<T>
where
    T: Send + Sync + 'static,
{
    /// Create a new AFast service instance
    ///
    /// # Arguments
    /// * `state` - Shared application state
    /// * `handlers` - List of registered handlers
    ///
    /// # Returns
    /// A new AFast instance
    pub fn new(state: Arc<T>, handlers: Vec<HandlerGeneric<T>>) -> Self {
        Self {
            state,
            handlers: Arc::new(handlers),
            js: false,
            doc: false,
        }
    }

    /// Enable or disable JS client output
    ///
    /// # Arguments
    /// * `enable` - `true` to enable JS client output, `false` to disable
    pub fn set_js(mut self, enable: bool) -> Self {
        self.js = enable;
        self
    }

    /// Enable or disable documentation output
    ///
    /// # Arguments
    /// * `enable` - `true` to enable doc output, `false` to disable
    pub fn set_doc(mut self, enable: bool) -> Self {
        self.doc = enable;
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

                        // Parse sequence number and handler ID
                        let seq: usize = u32::from_be_bytes([body[0], body[1], body[2], body[3]]) as usize;
                        let id: usize = u32::from_be_bytes([body[4], body[5], body[6], body[7]]) as usize;

                        // Call handler
                        let handler = &handlers[id];
                        let fut = (handler.func)(Arc::clone(&state), &body[8..]);
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

                    let seq: usize = u32::from_be_bytes([body[0], body[1], body[2], body[3]]);
                    let id: usize = u32::from_be_bytes([body[4], body[5], body[6], body[7]]);

                    let handler = &handlers[id];
                    let fut = (handler.func)(Arc::clone(&state), &body[8..]);
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
            let state = Arc::clone(&self.state);
            let handlers = Arc::clone(&self.handlers);

            let js_content = handlers
                .iter()
                .map(|h| h.js.clone())
                .collect::<Vec<String>>()
                .join("\n");

            let ts_content = handlers
                .iter()
                .map(|h| h.ts.clone())
                .collect::<Vec<String>>()
                .join("\n");

            let js_content = format!(
                "{}\n\nclass AFastClient {{\nconstructor(options){{this._options=options;if(!options.call){{throw new Error('call is required');}};this._call=options.call}}\n{}\n}}",
                js::JS,
                js_content
            );

            let ts_content = format!(
                "{}\n\nclass AFastClient {{\n_options:ClientOptions;_call:ClientCall;\n\nconstructor(options:ClientOptions){{this._options=options;if(!options.call){{throw new Error('call is required');}};this._call=options.call}}\n{}\n}}",
                js::TS,
                ts_content
            );

            // TCP listener for HTTP/WS
            let listener = tokio::net::TcpListener::bind(addr_http).await.unwrap();

            let app = axum::Router::new();

            // HTTP route /api
            #[cfg(feature = "http")]
            let app = app
                .route(
                    "/api",
                    axum::routing::post(
                        move |axum::Extension((state, handlers)): axum::Extension<(
                            Arc<T>,
                            Arc<Vec<HandlerGeneric<T>>>,
                        )>,
                              body: axum::body::Bytes| async move {
                            if body.len() < 4 {
                                return axum::response::Response::builder()
                                    .status(400)
                                    .body(http_body_util::Full::new(axum::body::Bytes::from(
                                        "Invalid request",
                                    )))
                                    .unwrap();
                            }
                            let id = u32::from_be_bytes([body[0], body[1], body[2], body[3]]);
                            let handler = &handlers[id as usize];
                            let fut = (handler.func)(Arc::clone(&state), &body[4..]);
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
                )
                .route(
                    "/js",
                    axum::routing::get(move || async move {
                        axum::response::Response::builder()
                            .status(200)
                            .header(
                                axum::http::header::CONTENT_TYPE,
                                "text/javascript; charset=utf-8",
                            )
                            .body(http_body_util::Full::new(axum::body::Bytes::from(
                                js_content,
                            )))
                            .unwrap()
                    }),
                )
                .route(
                    "/ts",
                    axum::routing::get(move || async move {
                        axum::response::Response::builder()
                            .status(200)
                            .header(
                                axum::http::header::CONTENT_TYPE,
                                "text/typescript; charset=utf-8",
                            )
                            .body(http_body_util::Full::new(axum::body::Bytes::from(
                                ts_content,
                            )))
                            .unwrap()
                    }),
                );

            // WebSocket route /ws
            #[cfg(feature = "ws")]
            let app = app.route(
                "/ws",
                axum::routing::any(
                    move |axum::Extension((state, handlers)): axum::Extension<(
                        Arc<T>,
                        Arc<Vec<HandlerGeneric<T>>>,
                    )>,
                          ws: axum::extract::ws::WebSocketUpgrade| async move {
                        ws.on_upgrade(|mut ws| async move {
                            loop {
                                let data = ws.recv().await.unwrap().unwrap();
                                if let axum::extract::ws::Message::Binary(body) = data {
                                    if body.len() < 8 {
                                        continue;
                                    }
                                    let seq: usize =
                                        u32::from_be_bytes([body[0], body[1], body[2], body[3]])
                                            as usize;
                                    let id: usize =
                                        u32::from_be_bytes([body[4], body[5], body[6], body[7]])
                                            as usize;
                                    let handler = &handlers[id];
                                    let fut = (handler.func)(Arc::clone(&state), &body[8..]);
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

            // Attach shared state and CORS
            let app = app.layer(axum::Extension((state, handlers))).layer(
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
