use std::sync::RwLock;

use device_query::{DeviceQuery, DeviceState};
use lazy_static::lazy_static;
use node_bindgen::derive::node_bindgen;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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
            println!("breaking loop");
            return true;
        } else if keys != prev_keys {
            let returnkeys: Vec<String> = keys
                .clone()
                .into_par_iter()
                .map(|x| format!("{}", x))
                .collect();
            returnjs(returnkeys);
        }
        prev_keys = keys;
    }
}

#[node_bindgen]
fn unload() {
    *SHOULDSTOP.write().unwrap() = true;
}

#[node_bindgen]
fn stop() {
    std::process::exit(0);
}
