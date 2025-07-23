mod server;
mod models;
mod database;
mod handlers;
mod routes;
mod helpers;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() {
    crate::server::run_server().await;
}
