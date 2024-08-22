mod application;
mod handlers;
mod server;
mod session;
mod user;

#[async_std::main]
async fn main() {
    let mut app = application::App::new();

    if let Err(e) = app.run().await {
        tracing::error!("Error: {}", e);
    }
}
