mod chess;
mod handlers;
mod models;

use crate::{
    chess::{
        bitboards::get_initial_bitboards,
        helpers::{get_all_moves, initialize_starting_position},
    },
    handlers::{board::get_all_moves_handler, moves::move_piece_handler},
    models::response::AppState,
};
use axum::{Error, Router, http::HeaderValue, routing::get};
use hyper::Method;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let router = create_router().await.unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn create_router() -> Result<Router, Error> {
    let board = Arc::new(Mutex::new(initialize_starting_position()));
    let bitboards = get_initial_bitboards();
    let state = AppState { board, bitboards };

    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("http://localhost:3000"))
        .allow_methods([Method::GET]);

    let router = Router::new()
        .route("/board", get(get_all_moves_handler))
        .route("/move/{origin}/{destination}", get(move_piece_handler))
        .with_state(state)
        .layer(cors);

    Ok(router)
}
