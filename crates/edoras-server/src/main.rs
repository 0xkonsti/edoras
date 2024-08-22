mod application;
mod server;

fn main() {
    let app = application::App::new();

    if let Err(e) = app.run().await {
        tracing::error!("Error: {}", e);
    }
}
