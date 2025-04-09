use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceState {
    pub device_id: String,
    pub connected_at: OffsetDateTime,
    pub last_seen: OffsetDateTime,
    pub telemetry: DeviceTelemetry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceTelemetry {
    pub temperature: f32,
    pub humidity: f32,
    pub cpu_usage: f32,
    pub memory_usage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebSocketMessage {
    DeviceConnect {
        device_id: String,
    },
    TelemetryUpdate {
        device_id: String,
        telemetry: DeviceTelemetry,
    },
    Command {
        device_id: String,
        command: DeviceCommand,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceCommand {
    Restart,
    UpdateConfig { config: String },
    RequestTelemetry,
} 