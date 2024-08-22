mod application;
mod server;
mod session;

#[async_std::main]
async fn main() {
    let app = application::App::new();

    if let Err(e) = app.run().await {
        tracing::error!("Error: {}", e);
    }
}
