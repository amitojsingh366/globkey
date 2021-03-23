use device_query::{DeviceQuery, DeviceState, Keycode};
use node_bindgen::derive::node_bindgen;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

struct Changed {
    key: Keycode,
    op: String,
}

#[node_bindgen]
async fn on<F: Fn(String, String)>(returnjs: F) {
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
                returnjs(i.op, format!("{}", i.key));
            });
        }
        prev_keys = keys;
    }
}

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
