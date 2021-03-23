use keyboard_query::{DeviceQuery, DeviceState};
use node_bindgen::derive::node_bindgen;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

struct Changed {
    key: u16,
    op: String,
}

#[node_bindgen]
async fn on<F: Fn(String, u64)>(returnjs: F) {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            let mut changed: Vec<Changed> = Vec::new();
            keys.clone().into_iter().for_each(|i| {
                if !prev_keys.contains(&i) {
                    changed.push(Changed {
                        key: i,
                        op: "keydown".to_string(),
                    });
                }
            });
            prev_keys.clone().into_iter().for_each(|i| {
                if !keys.contains(&i) {
                    changed.push(Changed {
                        key: i,
                        op: "keyup".to_string(),
                    });
                }
            });
            changed.into_iter().for_each(|i| {
                returnjs(i.op, i.key as u64);
            });
        }
        prev_keys = keys;
    }
}

#[node_bindgen]
async fn raw<F: Fn(Vec<u64>)>(returnjs: F) {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            let clonekeys: Vec<u64> = keys.clone().into_par_iter().map(|x| x as u64).collect();

            returnjs(clonekeys);
        }
        prev_keys = keys;
    }
}
