use dashmap::DashMap;
use fullrstack_common::{DeviceState, WebSocketMessage};
use tokio::sync::broadcast;
use std::sync::Arc;

pub struct DeviceManager {
    devices: Arc<DashMap<String, DeviceState>>,
    tx: broadcast::Sender<WebSocketMessage>,
}

impl DeviceManager {
    pub fn new(broadcast_capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(broadcast_capacity);
        Self {
            devices: Arc::new(DashMap::new()),
            tx,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<WebSocketMessage> {
        self.tx.subscribe()
    }

    pub async fn handle_message(&self, msg: WebSocketMessage) -> anyhow::Result<()> {
        match msg {
            WebSocketMessage::DeviceConnect { device_id } => {
                let state = DeviceState {
                    device_id: device_id.clone(),
                    connected_at: time::OffsetDateTime::now_utc(),
                    last_seen: time::OffsetDateTime::now_utc(),
                    telemetry: Default::default(),
                };
                self.devices.insert(device_id, state);
            }
            WebSocketMessage::TelemetryUpdate { device_id, telemetry } => {
                if let Some(mut state) = self.devices.get_mut(&device_id) {
                    state.telemetry = telemetry;
                    state.last_seen = time::OffsetDateTime::now_utc();
                }
            }
            _ => {}
        }
        
        // Broadcast message to all subscribers
        self.tx.send(msg)?;
        Ok(())
    }
} 