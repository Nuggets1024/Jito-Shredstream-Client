use tonic::transport::Channel;
use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient,
    SubscribeEntriesRequest,
};
use tokio::time::sleep;
use std::time::Duration;

pub struct ShredstreamClient {
    server_url: String,
}

impl ShredstreamClient {
    pub fn new(server_url: String) -> Self {
        Self { server_url }
    }

    pub async fn connect(&self) -> Result<ShredstreamProxyClient<Channel>, tonic::transport::Error> {
        println!("正在连接到Jito Shredstream: {}", self.server_url);
        ShredstreamProxyClient::connect(self.server_url.clone()).await
    }

    pub async fn subscribe_entries(
        &self,
        client: &mut ShredstreamProxyClient<Channel>
    ) -> Result<tonic::Streaming<jito_protos::shredstream::Entry>, Box<dyn std::error::Error>> {
        loop {
            let request = tonic::Request::new(SubscribeEntriesRequest {});
            match client.subscribe_entries(request).await {
                Ok(response) => return Ok(response.into_inner()),
                Err(e) => {
                    println!("订阅错误: {}，5秒后重试...", e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
} 