mod handlers;
mod models;

use std::sync::Arc;

use crate::{
    handlers::{board::get_all_moves_handler, moves::move_piece_handler},
    models::{bitboards::Bitboards, board::Board, response::AppState},
};
use axum::{Router, http::HeaderValue, routing::get};
use hyper::Method;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let router = create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

fn create_router() -> Router {
    Router::new()
        .route("/board", get(get_all_moves_handler))
        .route(
            "/move/{origin}/{destination}", //origin and destination are Positions
            get(move_piece_handler),
        )
        .with_state(create_state())
        .layer(create_cors())
}

fn create_state() -> AppState {
    let board = Arc::new(Mutex::new(Board::new()));
    let bitboards = Arc::new(Mutex::new(Bitboards::new()));
    AppState { board, bitboards }
}

fn create_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:3000"))
        .allow_methods([Method::GET])
}
