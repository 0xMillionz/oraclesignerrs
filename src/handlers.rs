use actix_web::{
    web,
    Error, 
};
use ed25519_dalek::{
    Signature,
    Signer
};
use pyth_sdk_solana::load_price_feed_from_account;
use solana_sdk::pubkey::Pubkey;
use serde::{ Serialize, Deserialize } ;
use std::str::FromStr;

use crate::AppData;

// we can only return/sign i32 since risc0 is 32bit
#[derive(Serialize, Deserialize)]
pub struct PriceRes {
    price: i32,
    expo: i32,
    price_sig: Vec<u8>,
    expo_sig: Vec<u8>,
    price_key: Vec<u8>,
    expo_key: Vec<u8>,
}

pub async fn pyth(
    pyth_product_key_str: web::Path<String>,
    app_data: web::Data<AppData>,
) -> Result<web::Json<PriceRes>, Error> {
    let rpc_client = app_data.client_pool.get_client(); 
    let price_keypair = &app_data.price_keypair; 
    let expo_keypair = &app_data.expo_keypair;

    let prod_key = Pubkey::from_str(&pyth_product_key_str).unwrap();

    let mut price_acct = rpc_client 
        .get_account(&prod_key)
        .unwrap();
    
    let price_feed = load_price_feed_from_account(&prod_key, &mut price_acct)
        .unwrap();

    
    // cast to i32 since arch is 32bit :(
    let pt: (i32, i32) = match price_feed.get_current_price() {
        Some(p) => {
            (p.price as i32, p.expo)
        },
        None => {
            (-1, -1)
        },
    };

    let signed_price: Signature = price_keypair.sign(&pt.0.to_le_bytes());
    let signed_expo: Signature = expo_keypair.sign(&pt.1.to_le_bytes()); 

    let json_res = PriceRes {
        price: pt.0,
        expo: pt.1,
        price_sig: bincode::serialize(&signed_price).unwrap(),
        expo_sig: bincode::serialize(&signed_expo).unwrap(),
        price_key: bincode::serialize(&price_keypair.public).unwrap(),
        expo_key: bincode::serialize(&expo_keypair.public).unwrap(),
    }; 
    
    Ok(web::Json(json_res))
}