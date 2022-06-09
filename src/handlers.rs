use actix_web::{
    web,
    Error,
};
use ed25519_dalek::{
    Keypair,
    Signature,
    Signer
};
use pyth_sdk_solana::load_price_feed_from_account;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use serde::{ Serialize, Deserialize } ;
use std::str::FromStr;


#[derive(Serialize, Deserialize)]
pub struct PriceRes {
    price: Vec<u8>,
    expo: Vec<u8>,
    p_key: Vec<u8>,
}

pub async fn pyth(
    pyth_product_key_str: web::Path<String>,
    keypair: web::Data<Keypair>,
) -> Result<web::Json<PriceRes>, Error> {
    // TODO(millionz): fix this making a client every request
    // tbh we should just make a system thread that update loops 
    // some 'static and then read it with the handler
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let prod_key = Pubkey::from_str(&pyth_product_key_str).unwrap();

    let mut price_acct = rpc_client
        .get_account(&prod_key)
        .unwrap();
    
    let price_feed = load_price_feed_from_account(&prod_key, &mut price_acct)
        .unwrap();

    // unsure that I can leave this negative but sanity checks in client fix I think?
    let pt: (i64, i32) = match price_feed.get_current_price() {
        Some(p) => {
            (p.price, p.expo)
        },
        None => {
            (-1, -1)
        },
    };

    // Okay now we sign things...
    let signed_price: Signature = keypair.sign(&pt.0.to_le_bytes());
    let signed_expo: Signature = keypair.sign(&pt.1.to_le_bytes()); 

    let json_res = PriceRes {
        price: bincode::serialize(&signed_price).unwrap(),
        expo: bincode::serialize(&signed_expo).unwrap(),
        p_key: bincode::serialize(&keypair.public).unwrap()
    }; 
    
    Ok(web::Json(json_res))
}