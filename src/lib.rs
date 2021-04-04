use device_query::{DeviceQuery, DeviceState};
use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn raw() -> Vec<String> {
    let device_state = DeviceState::new();
    let keys = device_state.get_keys();
    keys.clone().into_iter().map(|x| format!("{}", x)).collect()
}
