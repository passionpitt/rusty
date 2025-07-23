mod database;
mod handlers;
mod helpers;
mod models;
mod routes;
mod server;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    crate::server::run_server().await;
}
