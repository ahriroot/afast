pub enum Error {
    DecodeError,
    EncodeError,
    ClientError,
    ServerError,
    CustomError(u16, String),
}

impl Error {
    pub fn decode_error() -> Self {
        #[cfg(not(feature = "trace"))]
        {
            Self::DecodeError
        }
        #[cfg(feature = "trace")]
        Self::CustomError(400, capture_user_backtrace("400 DecodeError"))
    }

    pub fn encode_error() -> Self {
        #[cfg(not(feature = "trace"))]
        {
            Self::EncodeError
        }
        #[cfg(feature = "trace")]
        Self::CustomError(500, capture_user_backtrace("500 EncodeError"))
    }

    pub fn client_error() -> Self {
        #[cfg(not(feature = "trace"))]
        {
            Self::ClientError
        }
        #[cfg(feature = "trace")]
        Self::CustomError(400, capture_user_backtrace("400 Bad Request"))
    }

    pub fn server_error() -> Self {
        #[cfg(not(feature = "trace"))]
        {
            Self::ServerError
        }
        #[cfg(feature = "trace")]
        Self::CustomError(500, capture_user_backtrace("500 Internal Server Error"))
    }

    pub fn custom_error(code: u16, message: String) -> Self {
        Self::CustomError(code, message)
    }

    pub fn client(message: &str) -> Self {
        #[cfg(not(feature = "trace"))]
        {
            Self::CustomError(400, message.to_string())
        }
        #[cfg(feature = "trace")]
        Self::CustomError(400, capture_user_backtrace(message))
    }

    pub fn server(message: &str) -> Self {
        #[cfg(not(feature = "trace"))]
        {
            Self::CustomError(500, message.to_string())
        }
        #[cfg(feature = "trace")]
        Self::CustomError(500, capture_user_backtrace(message))
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DecodeError => write!(f, "Decode error"),
            Error::EncodeError => write!(f, "Encode error"),
            Error::ClientError => write!(f, "400 Bad Request"),
            Error::ServerError => write!(f, "500 Internal Server Error"),
            Error::CustomError(code, message) => write!(f, " {} Custom error: {}", code, message),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DecodeError => write!(f, "Decode error"),
            Error::EncodeError => write!(f, "Encode error"),
            Error::ClientError => write!(f, "400 Bad Request"),
            Error::ServerError => write!(f, "500 Internal Server Error"),
            Error::CustomError(code, message) => write!(f, " {} Custom error: {}", code, message),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(feature = "trace")]
pub fn capture_user_backtrace(message: &str) -> String {
    use backtrace::Backtrace;

    let mut out = Vec::new();
    out.push(message.to_string());

    let bt = Backtrace::new();

    for frame in bt.frames() {
        for sym in frame.symbols() {
            let path = sym.filename().map(|p| p.to_string_lossy().to_string());

            // 获取当前库名
            let self_crate = env!("CARGO_PKG_NAME");

            // 过滤规则：只保留用户项目源代码
            let is_user_code = match path {
                Some(ref p) => {
                    // 过滤掉标准库 / 注册表 / rustc 构建路径 / 你的库自身
                    !p.contains(".cargo/registry")
                        && !p.contains("rustc")
                        && !p.contains("std/src")
                        && !p.contains(self_crate)   // 不显示库自身
                        && p.contains("src/") // 用户工程文件基本都在 src/
                }
                None => false,
            };

            if !is_user_code {
                continue;
            }

            if let Some(p) = sym.filename() {
                let line = sym.lineno().unwrap_or(0);
                out.push(format!("{}:{}", p.display(), line));
            } else if let Some(name) = sym.name() {
                out.push(format!("{}", name));
            }
        }
    }

    out.join("\n")
}
