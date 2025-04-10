use fullrstack_common::{WebSocketMessage, DeviceTelemetry};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::{StreamExt, SinkExt};
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = Url::parse("ws://localhost:8080")?;
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to server");

    let (mut write, mut read) = ws_stream.split();

    // Send a test message
    let msg = WebSocketMessage::DeviceConnect {
        device_id: "test-device-1".to_string(),
    };
    let json = serde_json::to_string(&msg)?;
    write.send(Message::Text(json)).await?;
    println!("Sent connection message");

    // Read the response
    if let Some(response) = read.next().await {
        println!("Received: {:?}", response?);
    }

    Ok(())
} 