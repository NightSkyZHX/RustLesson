use lazy_static::lazy_static;
use mini_redis::LogLayer;
use std::{io, net::SocketAddr, process};

lazy_static! {
    static ref CLIENT: volo_gen::mini_redis::RedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::mini_redis::RedisServiceClientBuilder::new("volo-example")
            .layer_outer(LogLayer)
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    loop {
        let mut flag = false;
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.strip_suffix("\n").unwrap().strip_suffix("\r").unwrap().to_string();
        let str_vec: Vec<String> = input.split(' ').map(|str| str.to_string()).collect();
        // println!("{:?}", &str_vec);
        let mut req = volo_gen::mini_redis::RedisRequest {
            key: None,
            value: None,
            r#type: volo_gen::mini_redis::RequestType::Illegal,
        };
        
        match str_vec[0].as_str() {
            "PING" => {
                req = volo_gen::mini_redis::RedisRequest {
                    key: None,
                    value: Some(input.strip_prefix("PING ").unwrap().to_string().into()),
                    r#type: volo_gen::mini_redis::RequestType::Ping,
                }
            }
            "DEL" => {
                let mut temp = vec![];
                for i in 1..str_vec.len() {
                    temp.push(str_vec.get(i).unwrap().clone().into());
                }
                req = volo_gen::mini_redis::RedisRequest {
                    key: Some(temp),
                    value: None,
                    r#type: volo_gen::mini_redis::RequestType::Del,
                }
            }
            "GET" => {
                if str_vec.len() == 2 {
                    req = volo_gen::mini_redis::RedisRequest {
                        key: Some(vec![str_vec.get(1).unwrap().clone().into()]),
                        value: None,
                        r#type: volo_gen::mini_redis::RequestType::Get,
                    }
                }
            }
            "SET" => {
                if str_vec.len() == 3 {
                    req = volo_gen::mini_redis::RedisRequest {
                        key: Some(vec![str_vec.get(1).unwrap().clone().into()]),
                        value: Some(str_vec.get(2).unwrap().clone().into()),
                        r#type: volo_gen::mini_redis::RequestType::Set,
                    }
                }
            }
            "EXIT" => {
                flag = true;
                req = volo_gen::mini_redis::RedisRequest {
                    key: None,
                    value: None,
                    r#type: volo_gen::mini_redis::RequestType::Exit,
                }
            }
            _ => {}
        }

        let resp = CLIENT.redis_command(req).await;
        if flag {
            process::exit(0);
        }
        match resp {
            Ok(info) => tracing::info!("{:?}", info.value.unwrap()),
            Err(e) => tracing::error!("{:?}", e),
        }
    }
}
