use device_query::{DeviceQuery, DeviceState};
use node_bindgen::derive::node_bindgen;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[node_bindgen]
async fn raw<F: Fn(Vec<String>)>(returnjs: F) {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            let returnkeys: Vec<String> = keys.clone().into_par_iter().map(|x| format!("{}", x)).collect();
            returnjs(returnkeys);
        }
        prev_keys = keys;
    }
}

#[node_bindgen]
fn capturecombo() -> Vec<String> {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys && keys.len() == 2 {
            let returnkeys: Vec<String> = keys.clone().into_par_iter().map(|x| format!("{}", x)).collect();
            return returnkeys
        }
        prev_keys = keys;
    }
}
