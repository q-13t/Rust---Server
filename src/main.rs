pub mod http;
pub mod route;
mod server;
pub mod thread_pool;
pub mod transmitters;
pub mod utils;

use http::http_codes::StatusCode;
use http::http_content_types::*;
use http::http_methods::*;

use route::Route;

use route_macro_def::add_routes;

use std::{fs, thread};
use transmitters::request::*;
use transmitters::response::*;
use utils::logger::*;

extern crate route_macro;
extern crate route_macro_def;

const LOGGER: Logger = Logger {
    c_name: "Main",
    level: LogLevel::Debug,
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

fn main() {
    let routes = add_routes!(
        Route::new("/", index, HttpMethod::GET),
        Route::new("", appliances, HttpMethod::GET),
        Route::new("/sleep", sleep, HttpMethod::OPTIONS),
        Route::new("/sleep", sleep, HttpMethod::GET),
        Route::new("/options", options, HttpMethod::OPTIONS)
    );

    server::start(routes, "127.0.0.1", 8000, 10, LogLevel::Info);
}
