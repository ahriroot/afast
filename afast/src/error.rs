pub enum Error {
    DecodeError,
    EncodeError,
    HandlerError,
    ClientError(i32, String),
    ServerError(i32, String),
}

impl Error {
    pub fn decode_error() -> Self {
        Self::DecodeError
    }

    pub fn encode_error() -> Self {
        Self::EncodeError
    }

    pub fn handler_error() -> Self {
        Self::HandlerError
    }

    pub fn client_error(code: i32, message: String) -> Self {
        Self::ClientError(code, message)
    }

    pub fn server_error(code: i32, message: String) -> Self {
        Self::ServerError(code, message)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DecodeError => write!(f, "Decode error"),
            Error::EncodeError => write!(f, "Encode error"),
            Error::HandlerError => write!(f, "Handler error"),
            Error::ClientError(code, message) => write!(f, "Client error: {} - {}", code, message),
            Error::ServerError(code, message) => write!(f, "Server error: {} - {}", code, message),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DecodeError => write!(f, "Decode error"),
            Error::EncodeError => write!(f, "Encode error"),
            Error::HandlerError => write!(f, "Handler error"),
            Error::ClientError(code, message) => write!(f, "Client error: {} - {}", code, message),
            Error::ServerError(code, message) => write!(f, "Server error: {} - {}", code, message),
        }
    }
}

impl std::error::Error for Error {}
