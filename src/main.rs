use clientpool::ClientPool;
use dotenv::dotenv;
use std::env;
use actix_web::{
    web,
    App,
    HttpServer,
};
use ed25519_dalek::Keypair;

pub mod handlers;
pub mod clientpool;

// TODO(millionz): make a client pool to prevent 
// high volume jams
pub struct AppData {
    price_keypair: Keypair,
    expo_keypair: Keypair,
    client_pool: clientpool::ClientPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let s = HttpServer::new( move || {
        App::new()
            .route("/pyth/{product}", web::get().to(handlers::pyth))
            .app_data(web::Data::new(
                AppData{
                    price_keypair: Keypair::from_bytes(env::var("PRICEKEYPAIR")
                        .unwrap()
                        .as_bytes()
                    ).unwrap(),
                    expo_keypair: Keypair::from_bytes(env::var("EXPOKEYPAIR")
                        .unwrap()
                        .as_bytes()    
                    ).unwrap(),
                    client_pool: ClientPool::new(5),
                }
            ))
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    s.await
}
