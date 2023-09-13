#![feature(impl_trait_in_assoc_type)]

use std::{collections::HashMap, net::SocketAddr, sync::Mutex};

use mini_redis::S;

#[volo::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let addr = volo::net::Address::from(addr);

    volo_gen::mini_redis::RedisServiceServer::new(S {
        map: Mutex::new(HashMap::<String, String>::new()),
    })
    .run(addr)
    .await
    .unwrap();
}