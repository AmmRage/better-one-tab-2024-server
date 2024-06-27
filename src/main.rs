#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::env::args;
use std::net::SocketAddr;
use std::sync::Mutex;

use axum::{
    http::StatusCode,
    Json,
    Router, routing::{get, post},
    response::Response,
    http::HeaderMap,
};
use axum::extract::{ConnectInfo, FromRequest, Path, Query, Request};
use axum::middleware::Next;
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use crate::config::Config;
use crate::ip::Ips;
use crate::models::tabs::{TabGroup, Tabs};
use crate::models::update_response::update_response;
use crate::models::user::User;
use crate::util::{generate_random_string, get_tabs_from_file, read_lines_from_file, remove_user_token, save_tabs_to_file, save_token_to_file, try_get_username_token};

mod util;
mod logger;
mod config;
mod ip;

mod models {
    pub mod user; // 引入 greet_world 模块
    pub mod tabs; // 引入 greet_world 模块
    pub mod update_response;
}

lazy_static! {
    pub static ref IPS_INSTANCE: Mutex<Ips> = Mutex::new(Ips::new());
    pub static ref CONFIG_INSTANCE: Mutex<Config> = Mutex::new(Config::new());
}

#[tokio::main]
async fn main() {
    let params: Vec<String> = args().collect();
    if params.len() < 2 {
        println!("Usage: {} <port>", params[0]);
        println!("data directory should be created in the current directory");
        return;        
    }

    // check data directory
    let current_dir = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);
    let data_dir = current_dir.join("data");
    if !data_dir.exists() {
        println!("Create data directory: {:?} and users.txt", data_dir);
        return;
    }
    // check history directory
    let history_dir = data_dir.join("history");
    if !history_dir.exists() {
        std::fs::create_dir(history_dir.clone()).unwrap();
        println!("Create history directory: {:?}", history_dir);
    }
    
    let port = params[1].parse::<u16>().unwrap();
    println!("Listening on port {}", port);
    
    // initialize tracing
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
    ;



    let middle_ware = axum::middleware::from_fn (my_middleware);


    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/api/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user).options(options_handler))
        .route("/api/verify", post(verify_user).options(options_handler))
        .route("/api/user/:username/logout", post(logout_user).options(options_handler))
        .route("/api/user/:username", get(get_user_info))
        .route("/api/user/:username/tabs", post(update_tabs).options(options_handler))
        .route("/api/user/:username/tabs", get(get_tabs))
        .layer(middle_ware)
        .layer(cors)
        ;


    // run our app with hyper, listening globally on port 3000
    match tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        Ok(listener) => {
            match axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await {
                Ok(_) => {
                    println!("Server started");
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

async fn my_middleware(
    ConnectInfo(socket_addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    {
        let settings = &(CONFIG_INSTANCE.lock().unwrap().settings);
        if settings.enable_region_block == true {
            let ip_str = headers
                .get("x-forwarded-for")
                .and_then(|value| value.to_str().ok())
                .unwrap_or("127.0.0.1");

            // println!("Client IP: {}", client_ip);
            // let ip_str = socket_addr.ip().to_string();            // println!("Client IP: {}", client_ip);
            // let ip_str = socket_addr.ip().to_string();
            let ip_parts: Vec<&str> = ip_str.split(".").collect();
            let ip_u32 = ip_parts[0].parse::<u32>().unwrap() * 256 * 256 * 256 + ip_parts[1].parse::<u32>().unwrap() * 256 * 256 + ip_parts[2].parse::<u32>().unwrap() * 256 + ip_parts[3].parse::<u32>().unwrap();

            let region_code = IPS_INSTANCE.lock().unwrap().get_region(ip_u32);
            println!("{} - {} {} {}", socket_addr, request.method(), request.uri().path(), region_code);

            if !settings.contains_region(&region_code) {
                let forbidden_message = format!("Forbidden region: {}", region_code);
                let forbidden_response = Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(axum::body::Body::from(forbidden_message))
                    .unwrap();
                return forbidden_response;
            }
        }
    }

    let response = next.run(request).await;
    response
}

// basic handler that responds with a static string
async fn root() -> (StatusCode, Json<String>) {
    let message = format!("version: {}", env!("CARGO_PKG_VERSION"));
    (StatusCode::OK, Json(message))
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        username: payload.username,
        password: "password".to_string(),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

async fn verify_user(
    Json(payload): Json<User>,
) -> (StatusCode, Json<String>) {
    let filename = "users.txt";
    let mut users: Vec<User> = Vec::new();
    let mut error_message = String::from("OK");
    match read_lines_from_file(filename) {
        Ok(read_users) => {
            users = read_users;
        }
        Err(e) => {
            error_message = format!("Error reading file {}: {}", filename, e);
        }
    }

    if users.len() == 0 {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json("No users".to_string()));
    }

    if error_message != "OK" {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_message.to_string()));
    }

    for user in users {
        if user.username == payload.username && user.password == payload.password {
            let token = generate_random_string(32);
            save_token_to_file(format!("{}.txt", user.username), token.clone()).unwrap();
            return (StatusCode::OK, Json(token));
        }
    }

    (StatusCode::UNAUTHORIZED, Json("Incorrect username or password".to_string()))
}

async fn logout_user(
    Path(username): Path<String>, Json(payload): Json<String>,
) -> (StatusCode, Json<String>) {
    let result = try_get_username_token(&username, payload.to_string());
    if result {
        let _ = remove_user_token(&username);
        return (StatusCode::OK, Json("OK".to_string()));
    }

    (StatusCode::UNAUTHORIZED, Json("Not found token".to_string()))
}

async fn update_tabs(
    Path(username): Path<String>, Json(payload): Json<Tabs>
) -> (StatusCode, Json<update_response>) {
    let tabs = payload.tabs;
    let token = payload.token;
    let check_token = try_get_username_token(&username, token.to_string());
    if !check_token {
        return (StatusCode::UNAUTHORIZED, Json(update_response {
            message: "Not found token".to_string(),
            updated_at: chrono::Utc::now()
        }));
    }

    let json_str = serde_json::to_string(&tabs).unwrap();
    let filename = format!("{}.json", username);
    return match save_tabs_to_file(&filename, json_str) {
        Ok(()) => {
            (StatusCode::OK, Json(update_response {
                message: "OK".to_string(),
                updated_at: chrono::Utc::now()
            }))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(update_response {
                message: format!("Error saving file {}: {}", filename, e),
                updated_at: chrono::Utc::now()
            }))
        }
    }
}

async fn get_user_info(Path(username): Path<String>, Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<String>) {
    let token = params.get("token").unwrap();
    let result = try_get_username_token(&username, token.to_string());
    if result {
        return (StatusCode::OK, Json("OK".to_string()));
    }

    (StatusCode::UNAUTHORIZED, Json("Not found token".to_string()))
}

async fn options_handler() -> Response {
    Response::builder()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body(axum::body::Body::empty())
        .unwrap()
}


async fn get_tabs(Path(username): Path<String>, Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<Tabs>) {
    let token = params.get("token").unwrap();
    let result = try_get_username_token(&username, token.to_string());
    if result {
        let filename = format!("{}.json", username);
        return match get_tabs_from_file(filename) {
            Ok(tabs) => {
                // println!("tabs: {}", tabs);
                let tabs: Vec<TabGroup> = serde_json::from_str(&tabs).unwrap();                
                (StatusCode::OK, Json(Tabs {
                    tabs: tabs,
                    token: "".to_string()
                }))
            }
            Err(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR , Json(Tabs {
                    tabs: Vec::new(),
                    token: "".to_string()
                }))
            }
        }
    }

    (StatusCode::UNAUTHORIZED, Json(Tabs {
        tabs: Vec::new(),
        token: "".to_string()
    }))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

