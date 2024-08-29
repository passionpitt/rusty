mod server;
mod models;
mod database;
mod handlers;
mod routes;
mod helpers;

#[tokio::main]
async fn main() {
    crate::server::run_server().await;
}
