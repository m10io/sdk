use futures_core::Stream;
use futures_util::{SinkExt as _, StreamExt};
use std::pin::Pin;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::transport::{Endpoint, Uri};

use crate::error::{M10Error, M10Result};
use m10_protos::prost::Message;
use m10_protos::sdk;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message as WSMessage;

#[derive(Clone)]
pub struct WSClient {
    endpoint: Endpoint,
}

impl WSClient {
    pub fn new(endpoint: Endpoint) -> Self {
        Self { endpoint }
    }

    pub async fn observe_with_request<T, F>(
        &self,
        ep: &str,
        req: sdk::RequestEnvelope,
        f: F,
    ) -> M10Result<Pin<Box<dyn Stream<Item = M10Result<T>> + Send + Sync + 'static>>>
    where
        F: FnMut(Vec<u8>) -> M10Result<T> + Send + Sync + 'static,
    {
        let (msg_tx, msg_rx) = mpsc::unbounded_channel();

        tokio::spawn({
            let msg_tx = msg_tx.clone();
            let base_url = self.endpoint.uri().clone();
            let endpoint = ep.to_string().clone();

            async move {
                if let Err(err) = observe_msgs(msg_tx, req, base_url, endpoint).await {
                    eprintln!("Failed to spawn WebSocket client thread: {}", err);
                }
            }
        });

        Ok(Box::pin(UnboundedReceiverStream::new(msg_rx).map(f)))
    }
}

async fn observe_msgs(
    msg_tx: UnboundedSender<Vec<u8>>,
    req: sdk::RequestEnvelope,
    base_url: Uri,
    endpoint: String,
) -> M10Result<()> {
    let (mut ws, _) = connect_async(format!("{}ledger/ws/observe/{}", base_url, endpoint))
        .await
        .map_err(M10Error::from)?;

    let mut req_body = vec![];
    req.encode(&mut req_body).expect("Failed to encode");

    ws.send(req_body.into()).await.map_err(M10Error::from)?;

    while let Some(msg) = ws.next().await {
        match msg {
            Ok(WSMessage::Binary(bin)) => {
                if msg_tx.send(bin).is_err() {
                    break;
                }
            }

            Ok(WSMessage::Ping(_)) => {
                ws.send(WSMessage::Pong(Vec::new()))
                    .await
                    .map_err(M10Error::from)?;
            }

            Err(e) => {
                eprintln!(
                    "Error during listening messages from the WebSocket connection: {:?}",
                    e
                );
                return Err(M10Error::WsError(e));
            }

            _ => {}
        }
    }

    Ok(())
}
