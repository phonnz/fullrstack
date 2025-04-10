use criterion::{criterion_group, criterion_main, Criterion};
use async_tungstenite::tokio::connect_async;
use fullrstack_common::{WebSocketMessage, DeviceTelemetry};
use tokio::runtime::Runtime;
use url::Url;
use fake::{Fake, Faker};

async fn simulate_device(device_id: String, message_count: usize) {
    let url = Url::parse("ws://127.0.0.1:8080").unwrap();
    let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    
    // Connect device
    let connect_msg = WebSocketMessage::DeviceConnect { device_id: device_id.clone() };
    ws_stream.send(serde_json::to_string(&connect_msg).unwrap().into()).await.unwrap();
    
    // Send telemetry messages
    for _ in 0..message_count {
        let telemetry = DeviceTelemetry {
            temperature: (20.0..30.0).fake(),
            humidity: (40.0..80.0).fake(),
            cpu_usage: (0.0..100.0).fake(),
            memory_usage: (0.0..100.0).fake(),
        };
        
        let msg = WebSocketMessage::TelemetryUpdate {
            device_id: device_id.clone(),
            telemetry,
        };
        
        ws_stream.send(serde_json::to_string(&msg).unwrap().into()).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }
}

fn load_test(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    
    c.bench_function("1000_concurrent_devices", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut handles = vec![];
                
                // Spawn 1000 concurrent device connections
                for i in 0..1000 {
                    let device_id = format!("bench-device-{}", i);
                    handles.push(tokio::spawn(simulate_device(device_id, 10)));
                }
                
                // Wait for all devices to complete
                for handle in handles {
                    handle.await.unwrap();
                }
            });
        });
    });
}

criterion_group!(benches, load_test);
criterion_main!(benches); 