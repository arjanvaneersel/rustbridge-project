use std::env;
use simple_server::Server;

mod templates;

fn get_server_port() -> String {
    match env::var("PORT") {
        Ok(p) => p,
        Err(_) => String::from("7878"),
    }
}

fn main() {
    let app = Server::new(|_request, mut response| {
        //let html = String::from("<html><head><title>Rustbridge</title></head><body><h1>Hello Rust!</h1></body></html>").into_bytes();
        let motivation = templates::motivation();
        Ok(response.header("Content-Type", "text/html; charset=utf-8").body(motivation)?)
    });

    let host = "0.0.0.0";
    let port = get_server_port();
    println!("Starting server at {}:{}", host, port);
    app.listen(host, &port);
}
