use axum::extract::State;
use axum::routing::get;
use axum::{serve, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::spawn;

pub async fn create<const N: usize>(ports: [u16; N]) {
    for port in ports {
        spawn(bind(port));
    }
}

pub async fn bind(port: u16) {
    let router = Router::new().route("/", get(index)).with_state(port);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();
    serve(listener, router.into_make_service()).await.unwrap();
}

async fn index(State(port): State<u16>) -> String {
    format!("{}", port)
}
