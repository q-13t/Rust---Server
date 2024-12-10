# Rust - Server

This is a simple Rust server aimed to improve my understanding of Rust programming language and web development.

## Features

- Multithreading
- Logging
- Error handling
- HTTP server
- Custom routes
- Custom pre-request and pre-response handlers
- Static files
- Flexible Request and Response types
- RUST API
- Enum definitions

## File Structure

```rust
src
├── main.rs
├── http
│   ├── content_type.rs
│   ├── http_content_types.rs
│   ├── http_methods.rs
│   ├── http_codes.rs
│   ├── http_version.rs
│   └── mod.rs
├── transmitters
│   ├── mod.rs
│   └── request.rs
│   └── response.rs
├── utils
│   └── mod.rs
│   └── logger.rs
├── route.rs
├── server.rs
├── thread_pool.rs
└── types.rs
```

# Sample Usage

```rust

const LOG_LEVEL: LogLevel = LogLevel::Info;

const LOGGER: Logger = Logger {
    c_name: "Main",
    level: LOG_LEVEL,
};

fn appliances(data: Request) -> Response {
    LOGGER.debug(&["Responding to ", data.path.as_str()]);
    let resource_type = data.path.as_str();
    let data = match fs::read_to_string("./static/".to_owned() + resource_type) {
        Ok(data) => data,
        Err(e) => {
            LOGGER.error(e.to_string().as_str(), &["Error reading file"]);
            return Response::new(
                StatusCode::NotFound,
                ContentType::TextHtmlCharsetUtf8,
                Vec::new(),
                Vec::new(),
                "".to_string(),
            );
        }
    };
    Response::new(
        StatusCode::Ok,
        ContentType::parse_file_name(resource_type),
        Vec::new(),
        Vec::new(),
        data,
    )
}

fn index(_: Request) -> Response {
    Response::new(
        StatusCode::Ok,
        ContentType::TextHtmlCharsetUtf8,
        Vec::new(),
        Vec::new(),
        fs::read_to_string("./static/index.html").unwrap(),
    )
}

fn sleep(_: Request) -> Response {
    thread::sleep(std::time::Duration::from_secs(5));
    Response::new(
        StatusCode::Ok,
        ContentType::ApplicationJson,
        Vec::new(),
        Vec::new(),
        "{\"token\": true}".to_string(),
    )
}

fn options(_: Request) -> Response {
    Response::new(
        StatusCode::Ok,
        ContentType::TextPlainCharsetUtf8,
        Vec::new(),
        Vec::new(),
        "".to_string(),
    )
}

/// Custom pre-response handler
impl PreRequest for PreRequestHandler {
    fn call(&self, _: String, request: Request) -> Request {
        LOGGER.info(&["Pre-request called for ", request.path.as_str()]);
        request
    }
}

/// Custom pre-response handler
impl PreResponse for PreResponseHandler {
    fn call(&self, _: String, response: Response) -> Response {
        LOGGER.info(&[
            "Pre-response called for ",
            get_status_line(response.status).as_str(),
        ]);
        response
    }
}

fn main() {
    server::start(
        add_routes!(
            Route::new("/", index, HttpMethod::GET),
            Route::new("", appliances, HttpMethod::GET),
            Route::new("/sleep", sleep, HttpMethod::OPTIONS),
            Route::new("/sleep", sleep, HttpMethod::GET),
            Route::new("/options", options, HttpMethod::OPTIONS)
        ), // Add your routes here
        "127.0.0.1", // The IP address to listen on
        8000, // The port to listen on
        10, // The number of threads to use
        LOG_LEVEL, // The log level to use 
        Some(PreRequestHandler), // The pre request handler to use
        Some(PreResponseHandler), // The pre response handler to use
    );
}
```

# Any suggestions or contributions are welcome