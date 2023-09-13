#![feature(impl_trait_in_assoc_type)]

use std::{collections::HashMap, process, sync::Mutex};

use anyhow::{Error, Ok};

pub struct S {
    pub map: Mutex<HashMap<String, String>>,
}

#[volo::async_trait]
impl volo_gen::mini_redis::RedisService for S {
    async fn redis_command(
        &self,
        req: volo_gen::mini_redis::RedisRequest,
    ) -> ::core::result::Result<volo_gen::mini_redis::RedisResponse, ::volo_thrift::AnyhowError>
    {
        match req.r#type {
            volo_gen::mini_redis::RequestType::Del => {
                let mut count: usize = 0;
                for faststr in req.key.unwrap() {
                    if let Some(_) = self.map.lock().unwrap().remove(&faststr.to_string()) {
                        count += 1;
                    }
                }
                return Ok(volo_gen::mini_redis::RedisResponse {
                    value: Some(format!("(integer) {}", count).into()),
                    r#type: volo_gen::mini_redis::ResponseType::Value,
                });
            }
            volo_gen::mini_redis::RequestType::Get => {
                if let Some(str) = self
                    .map
                    .lock()
                    .unwrap()
                    .get(&req.key.unwrap().get(0).unwrap().to_string())
                {
                    return Ok(volo_gen::mini_redis::RedisResponse {
                        value: Some(str.clone().into()),
                        r#type: volo_gen::mini_redis::ResponseType::Value,
                    });
                } else {
                    return Ok(volo_gen::mini_redis::RedisResponse {
                        value: Some(format!("nil").into()),
                        r#type: volo_gen::mini_redis::ResponseType::Value,
                    });
                }
            }
            volo_gen::mini_redis::RequestType::Set => {
                let _ = self.map.lock().unwrap().insert(
                    req.key.unwrap().get(0).unwrap().to_string(),
                    req.value.unwrap().to_string(),
                );
                return Ok(volo_gen::mini_redis::RedisResponse {
                    value: Some(format!("OK",).into()),
                    r#type: volo_gen::mini_redis::ResponseType::Ok,
                });
            }
            volo_gen::mini_redis::RequestType::Ping => {
                return Ok(volo_gen::mini_redis::RedisResponse {
                    value: req.value,
                    r#type: volo_gen::mini_redis::ResponseType::Value,
                });
            }
            volo_gen::mini_redis::RequestType::Exit => {
                println!("bye");
                process::exit(0);
            }
            volo_gen::mini_redis::RequestType::Publish => {}
            volo_gen::mini_redis::RequestType::Subscribe => {}
            volo_gen::mini_redis::RequestType::Illegal => {}
        }
        Ok(Default::default())
    }
}

#[derive(Clone)]
pub struct LogService<S>(S);

#[volo::service]
impl<Cx, Req, S> volo::Service<Cx, Req> for LogService<S>
where
    Req: std::fmt::Debug + Send + 'static,
    S: Send + 'static + volo::Service<Cx, Req> + Sync,
    S::Response: std::fmt::Debug,
    S::Error: std::fmt::Debug + From<Error>,
    Cx: Send + 'static,
{
    async fn call(&self, cx: &mut Cx, req: Req) -> Result<S::Response, S::Error> {
        let now = std::time::Instant::now();
        tracing::debug!("Received request {:?}", &req);
        let info = format!("{:?}", &req);
        println!("{}", info);
        if info.contains("Illegal") {
            return Err(S::Error::from(Error::msg("Illegal instruction")));
        }
        let resp = self.0.call(cx, req).await;
        tracing::debug!("Sent response {:?}", &resp);
        tracing::info!("Request took {}ms", now.elapsed().as_millis());
        resp
    }
}

pub struct LogLayer;

impl<S> volo::Layer<S> for LogLayer {
    type Service = LogService<S>;

    fn layer(self, inner: S) -> Self::Service {
        LogService(inner)
    }
}