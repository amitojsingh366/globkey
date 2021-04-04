use std::sync::RwLock;

use device_query::{DeviceQuery, DeviceState};
use lazy_static::lazy_static;
use node_bindgen::derive::node_bindgen;

lazy_static! {
    static ref SHOULDSTOP: RwLock<bool> = RwLock::new(false);
}

#[node_bindgen]
async fn raw<F: Fn(Vec<String>)>(returnjs: F) -> bool {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    loop {
        let keys = device_state.get_keys();
        if *SHOULDSTOP.read().unwrap() {
            return true;
        } else if keys != prev_keys {
            let returnkeys: Vec<String> =
                keys.clone().into_iter().map(|x| format!("{}", x)).collect();
            returnjs(returnkeys);
        }
        prev_keys = keys;
    }
}

#[node_bindgen]
fn get_keys() -> Vec<String> {
    let device_state = DeviceState::new();
    let keys = device_state.get_keys();
    keys.clone().into_iter().map(|x| format!("{}", x)).collect()
}

#[node_bindgen]
fn unload() {
    *SHOULDSTOP.write().unwrap() = true;
}
