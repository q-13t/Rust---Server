pub mod http;
pub mod route;
pub mod thread_pool;
pub mod transmitters;
pub mod utils;
use http::http_codes::get_status_line;
use http::http_codes::StatusCode;
use http::http_content_types::*;
use http::http_methods::*;
use regex::Regex;
use route::Route;
use route_macro::get;
use route_macro_def::add_routes;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{SocketAddrV4, TcpListener, TcpStream},
    thread,
};
use thread_pool::ThreadPool;
use transmitters::request::*;
use transmitters::response::*;
use utils::logger::*;

extern crate route_macro;
extern crate route_macro_def;

const LOG_LEVEL: LogLevel = LogLevel::Debug;
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

use std::sync::RwLock;

lazy_static::lazy_static! {
    static ref ROUTES: RwLock<Vec<Route>> = RwLock::new(Vec::new());
}

fn main() {
    let address: SocketAddrV4 = SocketAddrV4::new("127.0.0.1".parse().unwrap(), 8000);
    let listener = TcpListener::bind(address).expect("Unable to start server");
    let thread_pool = ThreadPool::new(5);
    let routes = add_routes!(
        Route::new("/", index, HttpMethod::GET),
        Route::new("", appliances, HttpMethod::GET),
        Route::new("/sleep", sleep, HttpMethod::OPTIONS),
        Route::new("/sleep", sleep, HttpMethod::GET),
        Route::new("/options", options, HttpMethod::OPTIONS)
    );
    *ROUTES.write().unwrap() = routes;

    LOGGER.info(&[
        "Server started",
        "on",
        ("http://".to_string() + &listener.local_addr().unwrap().to_string()).as_str(),
    ]);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread_pool.execute(|| handle_connection(stream));
            }
            Err(e) => {
                LOGGER.error(e.to_string().as_str(), &["Unable to connect to client"]);
            }
        }
    }
}

fn submit(mut stream: &TcpStream, route: &Route, data: Request) -> Result<(), std::io::Error> {
    let resp = route.call(data).prepare();
    LOGGER.debug(&["Sending response", resp.as_str()]);
    match stream.write_all(resp.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => {
            LOGGER.error(e.to_string().as_str(), &["Unable to write to client"]);
            Err(e)
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // TODO: Adapt to the Request / Response system

    let buf_reader = BufReader::new(&mut stream);

    let request = Request::parse(
        buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<String>(),
    );

    let routes = ROUTES.read().unwrap();
    let appliances_regex = Regex::new(r"(.css|.html|.js|.ico)$").unwrap();

    let mut handled_successfully = false;

    if request.method == HttpMethod::GET && appliances_regex.is_match(request.path.as_str()) {
        match submit(&stream, &routes[1], request) {
            Ok(_) => {
                handled_successfully = true;
            }
            Err(e) => {
                LOGGER.error(e.to_string().as_str(), &["Unable to send response"]);
                stream
                    .write_all(get_status_line(StatusCode::InternalServerError).as_bytes())
                    .unwrap();
            }
        }
    } else {
        let mut found = false;
        for route in routes.iter() {
            if route.method == request.method && route.path == request.path {
                match submit(&stream, route, request) {
                    Ok(_) => {
                        handled_successfully = true;
                        found = true;
                        LOGGER.info(&[
                            "Response sent",
                            stream.peer_addr().unwrap().to_string().as_str(),
                        ]);
                    }
                    Err(e) => {
                        LOGGER.error(e.to_string().as_str(), &["Unable to send response"]);
                        stream
                            .write_all(get_status_line(StatusCode::InternalServerError).as_bytes())
                            .unwrap();
                    }
                }
                break;
            }
        }
        if !found {
            stream
                .write_all(get_status_line(StatusCode::NotFound).as_bytes())
                .unwrap();
        }
    }

    if !handled_successfully {
        stream
            .write_all(get_status_line(StatusCode::InternalServerError).as_bytes())
            .unwrap();
    }
    stream.flush().unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap();
    return;
}
