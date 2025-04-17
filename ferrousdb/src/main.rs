use storage::Store;

fn main() {
    let port = 8080;
    let store = Store::init();
    match server::serve(port, store) {
        Ok(_) => {
            println!("Server started on port: {}", port);
        }
        Err(e) => {
            eprintln!("ERROR: Failed to start server: {}", e);
        }
    }
}
