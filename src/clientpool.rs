use rand::prelude::SliceRandom;
use solana_client::rpc_client::RpcClient;

// this can probably be a type alias but whatever
pub struct ClientPool {
    pool: Vec<RpcClient>
}

impl ClientPool {
    pub fn new(client_num: u32) -> ClientPool {
        let mut clients: Vec<RpcClient>= Vec::new();

        clients.push(RpcClient::new("https://api.mainnet-beta.solana.com".to_string()));
        for _ in 1..client_num {
            clients.push(RpcClient::new("https://api.mainnet-beta.solana.com".to_string()))
        }

        ClientPool{
            pool: clients
        }
    }

    pub fn get_client(&self) -> &RpcClient {
        self.pool
            .choose(&mut rand::thread_rng())
            .map(|client| client.clone())
            .unwrap()
    }
}
