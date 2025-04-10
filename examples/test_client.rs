use fullrstack_common::{WebSocketMessage, DeviceTelemetry};
use tokio_tungstenite::connect_async;
use url::Url;
use futures::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("ws://localhost:8080")?;
    let (ws_stream, _) = connect_async(url).await?;
    let (mut write, mut read) = ws_stream.split();

    // Connect as a device
    let connect_msg = WebSocketMessage::DeviceConnect {
        device_id: "test-device-1".to_string(),
    };
    
    write.send(serde_json::to_string(&connect_msg)?.into()).await?;

    // Send telemetry every second
    let telemetry_task = tokio::spawn(async move {
        loop {
            let telemetry = DeviceTelemetry {
                temperature: 25.5,
                humidity: 60.0,
                cpu_usage: 45.0,
                memory_usage: 70.0,
            };

            let msg = WebSocketMessage::TelemetryUpdate {
                device_id: "test-device-1".to_string(),
                telemetry,
            };

            write.send(serde_json::to_string(&msg)?.into()).await?;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        #[allow(unreachable_code)]
        Ok::<_, Box<dyn std::error::Error>>(())
    });

    // Print received messages
    while let Some(msg) = read.next().await {
        println!("Received: {:?}", msg?);
    }

    telemetry_task.abort();
    Ok(())
} 