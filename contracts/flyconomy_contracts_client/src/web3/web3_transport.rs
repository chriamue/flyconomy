use std::ops::Deref;
#[cfg(not(target_arch = "wasm32"))]
use web3::transports::WebSocket;
#[cfg(target_arch = "wasm32")]
use web3::transports::Http;
#[cfg(target_arch = "wasm32")]
use web3::transports::eip_1193::Eip1193;

pub enum TransportType {
    #[cfg(not(target_arch = "wasm32"))]
    WebSocket(String),
    #[cfg(target_arch = "wasm32")]
    Http(String),
    #[cfg(target_arch = "wasm32")]
    Eip1193
}

#[derive(Debug, Clone)]
pub enum Web3Transport {
    #[cfg(not(target_arch = "wasm32"))]
    WebSocket(WebSocket),
    #[cfg(target_arch = "wasm32")]
    Http(Http),
    #[cfg(target_arch = "wasm32")]
    Eip1193(Eip1193),
}

impl Web3Transport {
    async fn new(transport_type: TransportType) -> Result<Self, Box<dyn std::error::Error>> {
        match transport_type {
            #[cfg(not(target_arch = "wasm32"))]
            TransportType::WebSocket(node_url) => Ok(Web3Transport::WebSocket(WebSocket::new(&node_url).await?)),
            #[cfg(target_arch = "wasm32")]
            TransportType::Http(node_url) => Ok(Web3Transport::Http(Http::new(&node_url)?)),
            #[cfg(target_arch = "wasm32")]
            TransportType::Eip1193 => {
                let provider = web3::transports::eip_1193::Provider::default().unwrap().unwrap();
                Ok(Web3Transport::Eip1193(Eip1193::new(provider)))
            },
        }
    }
}
