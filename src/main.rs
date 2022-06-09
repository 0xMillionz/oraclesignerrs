use dotenv::dotenv;
use std::env;
use actix_web::{
    web,
    App,
    HttpServer,
};
use ed25519_dalek::Keypair;

pub mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let s = HttpServer::new( move || {
        App::new()
            .route("/pyth/{product}", web::get().to(handlers::pyth))
            .app_data(web::Data::new(
                Keypair::from_bytes(env::var("KEYPAIR")
                    .unwrap()
                    .as_bytes()
                ).unwrap())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    s.await
}
