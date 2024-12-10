use std::{
    io::{BufRead, BufReader, Write},
    net::{SocketAddrV4, TcpListener, TcpStream},
    sync::OnceLock,
};

use regex::Regex;

use crate::{
    http::{http_codes::*, http_methods::*},
    route::Route,
    thread_pool::ThreadPool,
    utils::logger::Logger,
    LogLevel, Request, Response,
};
use std::sync::RwLock;

static LOG_LEVEL: OnceLock<LogLevel> = OnceLock::new();

/// Sets the log level
/// # Arguments
/// * `level` - The log level to set : LogLevel
pub fn set_log_level(level: LogLevel) {
    LOG_LEVEL.set(level).unwrap();
}
/// Returns the current log level
/// # Returns
/// * `LogLevel` - The current log level : LogLevel
pub fn get_log_level() -> LogLevel {
    LOG_LEVEL.get_or_init(|| LogLevel::Debug).clone()
}

lazy_static::lazy_static! {
    static ref ROUTES: RwLock<Vec<Route>> = RwLock::new(Vec::new());
}

#[allow(unused)]
/// Starts the server
/// Will listen to new connections coming to the specified address and port.
/// <br>
/// If the PreRequest and PreResponse handlers are provided, they will be used to handle the requests and responses before processing them further.  Tey provide more control over the flow of the server.
///
/// # Arguments
/// * `routes` - The routes to use : Vec<Route>
/// * `address` - The address to listen on : &str
/// * `port` - The port to listen on : u32
/// * `threads` - The number of threads to use : usize
/// * `log_level` - The log level to use : LogLevel
/// * `pre_request` - The pre request handler to use : Option<PreRequestHandler>
/// * `pre_response` - The pre response handler to use : Option<PreResponseHandler>
///
/// # Example
/// ``` rust
/// server::start(
///        add_routes!(
///            Route::new("/", index, HttpMethod::GET),
///            Route::new("", appliances, HttpMethod::GET),
///            Route::new("/sleep", sleep, HttpMethod::OPTIONS),
///            Route::new("/sleep", sleep, HttpMethod::GET),
///            Route::new("/options", options, HttpMethod::OPTIONS)
///        ),
///        "127.0.0.1",
///        8000,
///        10,
///        LogLevel::Info,
///        Some(PreRequestHandler),
///        Some(PreResponseHandler),
///    );
/// ```
pub fn start(
    routes: Vec<Route>,
    address: &str,
    mut port: u32,
    threads: usize,
    log_level: LogLevel,
    pre_request: Option<PreRequestHandler>,
    pre_response: Option<PreResponseHandler>,
) {
    set_log_level(log_level);
    let logger: Logger = Logger {
        c_name: "SERVER",
        level: get_log_level(),
    };
    if port > 65535 || port <= 0 {
        port = 8080;
    }
    ROUTES.write().unwrap().extend(routes);

    let address: SocketAddrV4 =
        SocketAddrV4::new(address.parse().unwrap(), port.try_into().unwrap());
    let listener = TcpListener::bind(address).expect("Unable to start server");
    let thread_pool = ThreadPool::new(threads as usize);

    logger.info(&[
        "Server started",
        "on",
        ("http://".to_string() + &listener.local_addr().unwrap().to_string()).as_str(),
    ]);
    for stream in listener.incoming() {
        let pre_request_clone = pre_request.clone();
        let pre_response_clone = pre_response.clone();
        match stream {
            Ok(stream) => {
                thread_pool.execute(|| {
                    handle_connection(stream, pre_request_clone, pre_response_clone);
                });
            }
            Err(e) => {
                logger.error(e.to_string().as_str(), &["Unable to connect to client"]);
            }
        }
    }
}
/// Submits a request to the client
/// # Arguments
/// * `stream` - The stream to write to : &TcpStream
/// * `route` - The route to use : &Route
/// * `data` - The data to send : Request
/// * `pre_response` - The pre response handler to use : Option<PreResponseHandler>
/// # Returns
/// * `Result<(), std::io::Error>`

fn submit(
    mut stream: &TcpStream,
    route: &Route,
    data: Request,
    pre_response: Option<PreResponseHandler>,
) -> Result<(), std::io::Error> {
    let logger: Logger = Logger {
        c_name: "SERVER",
        level: get_log_level(),
    };
    let resp = route.call(data);
    let response = match pre_response {
        Some(function) => function.call(stream.peer_addr().unwrap().to_string(), resp),
        None => resp,
    };
    let resp = response.prepare();
    logger.debug(&["Sending response", resp.as_str()]);
    match stream.write_all(resp.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => {
            logger.error(e.to_string().as_str(), &["Unable to write to client"]);
            Err(e)
        }
    }
}

/// Handles the connection with the clients. If the client sends the request this function will handle it in following flow:
/// <ol>
/// <li>Read request string</li>
/// <li>Parse the request</li>
/// <li>Execute if provided PreRequest function</li>
/// <li>Search for appropriate Route mapping</li>
/// <li>Call the submit function</li>
/// </ol>
///
/// If there is no mapping found, it will return a `404` response or if there is an error it will return a `500` response.
///
/// # Arguments
/// * `stream` - The stream to write to : &TcpStream
/// * `pre_request` - The pre request handler to use : Option<PreRequestHandler>
/// * `pre_response` - The pre response handler to use : Option<PreResponseHandler>
fn handle_connection(
    mut stream: TcpStream,
    pre_request: Option<PreRequestHandler>,
    pre_response: Option<PreResponseHandler>,
) {
    let logger: Logger = Logger {
        c_name: "SERVER",
        level: get_log_level(),
    };
    let buf_reader_string = BufReader::new(&mut stream)
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<String>();

    let request = match pre_request {
        Some(function) => function.call(
            stream.peer_addr().unwrap().to_string(),
            Request::parse(buf_reader_string),
        ),
        None => Request::parse(buf_reader_string),
    };

    let routes = ROUTES.read().unwrap();
    let appliances_regex = Regex::new(r"(.css|.html|.js|.ico)$").unwrap();

    let mut handled_successfully = false;

    if request.method == HttpMethod::GET && appliances_regex.is_match(request.path.as_str()) {
        match submit(&stream, &routes[1], request, pre_response.clone()) {
            Ok(_) => {
                handled_successfully = true;
            }
            Err(e) => {
                logger.error(e.to_string().as_str(), &["Unable to send response"]);
                stream
                    .write_all(get_status_line(StatusCode::InternalServerError).as_bytes())
                    .unwrap();
            }
        }
    } else {
        let mut found = false;
        for route in routes.iter() {
            if route.method == request.method && route.path == request.path {
                match submit(&stream, route, request, pre_response.clone()) {
                    Ok(_) => {
                        handled_successfully = true;
                        found = true;
                        logger.info(&[
                            "Response sent",
                            stream.peer_addr().unwrap().to_string().as_str(),
                        ]);
                    }
                    Err(e) => {
                        logger.error(e.to_string().as_str(), &["Unable to send response"]);

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

#[derive(Clone)]
pub struct PreRequestHandler;
#[derive(Clone)]
pub struct PreResponseHandler;

#[allow(unused)]
pub trait PreRequest: Send + Sync {
    fn call(&self, data: String, request: Request) -> Request;
}

#[allow(unused)]
pub trait PreResponse: Send + Sync {
    fn call(&self, data: String, request: Response) -> Response;
}
