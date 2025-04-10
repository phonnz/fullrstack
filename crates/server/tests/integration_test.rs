use async_tungstenite::tokio::connect_async;
use fullrstack_common::{DeviceState, DeviceTelemetry, WebSocketMessage};
use fullrstack_server::start_server;
use tokio::sync::oneshot;
use url::Url;

#[tokio::test]
async fn test_device_connection_flow() {
    // Start the server
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let server_handle = tokio::spawn(start_server("127.0.0.1:8080", shutdown_rx));
    
    // Connect as a device
    let url = Url::parse("ws://127.0.0.1:8080").unwrap();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    
    // Send device connection message
    let connect_msg = WebSocketMessage::DeviceConnect {
        device_id: "test-device-1".to_string(),
    };
    let msg_str = serde_json::to_string(&connect_msg).unwrap();
    ws_stream.send(msg_str.into()).await.unwrap();
    
    // Send telemetry update
    let telemetry_msg = WebSocketMessage::TelemetryUpdate {
        device_id: "test-device-1".to_string(),
        telemetry: DeviceTelemetry {
            temperature: 25.5,
            humidity: 60.0,
            cpu_usage: 45.0,
            memory_usage: 70.0,
        },
    };
    let msg_str = serde_json::to_string(&telemetry_msg).unwrap();
    ws_stream.send(msg_str.into()).await.unwrap();
    
    // Cleanup
    shutdown_tx.send(()).unwrap();
    server_handle.await.unwrap();
} 