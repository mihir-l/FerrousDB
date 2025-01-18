use storage::Store;

mod server;
mod storage;

#[tokio::main]
async fn main() {
    let port = 8080;
    let store = Store::init();
    match server::serve(port, store).await {
        Ok(_) => {
            println!("Server started on port: {}", port);
        }
        Err(e) => {
            eprintln!("ERROR: Failed to start server: {}", e);
        }
    }
}
