use keyboard_query::{DeviceQuery, DeviceState};
use node_bindgen::derive::node_bindgen;

struct Changed {
    key: u16,
    op: String
}

#[node_bindgen]
async fn on<F: Fn(String, u64)>(second: F) {
    let device_state = DeviceState::new();
    let mut prev_keys = vec![];
    loop {
        let keys = device_state.get_keys();
        if keys != prev_keys {
            let mut changed: Vec<Changed> = Vec::new();
            keys.clone().into_iter().for_each(|i| {
                if !prev_keys.contains(&i) {
                    changed.push(Changed {key: i, op: "keydown".to_string()});
                }
            });
            prev_keys.clone().into_iter().for_each(|i| {
                if !keys.contains(&i) {
                    changed.push(Changed {key: i, op: "keyup".to_string()});
                }
            });
            changed.into_iter().for_each(|i| {
                second(i.op, i.key as u64);
            });
        }
        prev_keys = keys;
    }
}
