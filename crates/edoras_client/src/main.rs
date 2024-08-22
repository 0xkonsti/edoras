mod application;

#[async_std::main]
async fn main() {
    let mut app = application::App::new();

    if let Err(e) = app.run().await {
        tracing::error!("Error: {}", e);
    }
}
