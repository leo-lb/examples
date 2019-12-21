#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use actix_files as fs;
use actix_web::{middleware, App, HttpServer};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("../openssl/key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("../openssl/cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            // enable logger
            //.wrap(middleware::Logger::default())
            .service(
                // static files
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
    })
    .bind_openssl("127.0.0.1:8181", builder)?
    .start()
    .await
}
