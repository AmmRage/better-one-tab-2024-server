mod util;

use std::collections::HashMap;
use std::env::args;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use axum::extract::{Path, Query};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use crate::models::user::User;
use crate::util::{generate_random_string, read_lines_from_file, save_token_to_file, try_get_username_token};

mod models {
    pub mod user; // 引入 greet_world 模块
}

#[tokio::main]
async fn main() {
    let params: Vec<String> = args().collect();
    if params.len() < 2 {
        println!("Usage: {} <port>", params[0]);
        return;        
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

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/api/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/api/verify", post(verify_user))
        .route("/api/user/:username", get(get_user_info))
        .layer(cors)
        ;


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
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

    (StatusCode::UNAUTHORIZED, Json("Not found token".to_string()))
}

async fn get_user_info(Path(username): Path<String>, Query(params): Query<HashMap<String, String>>) -> (StatusCode, Json<String>) {
    let token = params.get("token").unwrap();
    let result = try_get_username_token(username, token.to_string());
    if result {
        return (StatusCode::OK, Json("OK".to_string()));
    }

    (StatusCode::UNAUTHORIZED, Json("Not found token".to_string()))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

