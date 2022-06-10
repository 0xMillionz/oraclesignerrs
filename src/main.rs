use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use std::env;
use actix_web::{
    web,
    App,
    HttpServer,
};
use ed25519_dalek::Keypair;

pub mod handlers;

// TODO(millionz): make a client pool to prevent 
// high volume jams
pub struct AppData {
    app_keypair: Keypair,
    client: RpcClient,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let s = HttpServer::new( move || {
        App::new()
            .route("/pyth/{product}", web::get().to(handlers::pyth))
            .app_data(web::Data::new(
                AppData{
                    app_keypair: Keypair::from_bytes(env::var("KEYPAIR")
                        .unwrap()
                        .as_bytes()
                    ).unwrap(),
                    client: RpcClient::new("https://api.mainnet-beta.solana.com".to_string()),
                }
            ))
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    s.await
}
