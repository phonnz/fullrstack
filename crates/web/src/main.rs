use leptos::*;
use fullrstack_common::{DeviceState, WebSocketMessage};
use crate::wasm_bindgen::closure::Closure;
use leptos::wasm_bindgen::JsCast;
use js_sys::JsString;
use time;

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="dashboard">
            <h1>"Fullrstack Dashboard"</h1>
            <DeviceTable/>
        </div>
    }
}

#[component]
fn DeviceTable() -> impl IntoView {
    let (devices, set_devices) = create_signal(vec![]);
    
    // WebSocket connection
    spawn_local(async move {
        let ws = web_sys::WebSocket::new("ws://localhost:8080").unwrap();
        
        let onmessage = Closure::wrap(Box::new(move |e: web_sys::MessageEvent| {
            if let Ok(txt) = e.data().dyn_into::<JsString>() {
                if let Ok(msg) = serde_json::from_str::<WebSocketMessage>(&String::from(txt)) {
                    match msg {
                        WebSocketMessage::DeviceConnect { ref device_id } => {
                            let device = DeviceState {
                                device_id: device_id.to_string(),
                                connected_at: time::OffsetDateTime::now_utc(),
                                last_seen: time::OffsetDateTime::now_utc(),
                                telemetry: Default::default(),
                            };
                            set_devices.update(|devices| devices.push(device));
                        }
                        WebSocketMessage::TelemetryUpdate { ref device_id, ref telemetry } => {
                            set_devices.update(|devices| {
                                if let Some(device) = devices.iter_mut().find(|d| d.device_id == *device_id) {
                                    device.telemetry = telemetry.clone();
                                    device.last_seen = time::OffsetDateTime::now_utc();
                                }
                            });
                        }
                        _ => {}
                    }
                    log::info!("Received message: {:?}", msg);
                
                }
            }
        }) as Box<dyn FnMut(_)>);
        
        ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        onmessage.forget();
    });

    view! {
        <table class="device-table">
            <thead>
                <tr>
                    <th>"Device ID"</th>
                    <th>"Status"</th>
                    <th>"Last Seen"</th>
                    <th>"Temperature"</th>
                    <th>"CPU Usage"</th>
                </tr>
            </thead>
            <tbody>
                {move || devices.get().into_iter().map(|device: DeviceState| view! {
                    <tr>
                        <td>{device.device_id}</td>
                        <td>"Connected"</td>
                        <td>"Now"</td>
                        <td>{format!("{:.1}°C", device.telemetry.temperature)}</td>
                        <td>{format!("{:.1}%", device.telemetry.cpu_usage)}</td>
                    </tr>
                }).collect_view()}
            </tbody>
        </table>
    }
}

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    mount_to_body(|| view! { <App/> });
} 