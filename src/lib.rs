use parking_lot::{Mutex, RwLock};
use std::sync::mpsc;

use device_query::{DeviceQuery, DeviceState};
use node_bindgen::derive::node_bindgen;
use once_cell::sync;

static DEVICEMPSC: sync::Lazy<(
    Mutex<mpsc::Sender<Vec<String>>>,
    Mutex<mpsc::Receiver<Vec<String>>>,
)> = sync::Lazy::new(|| {
    let (tx, rx) = mpsc::channel::<Vec<String>>();
    (Mutex::new(tx), Mutex::new(rx))
});

static SHOULDSTOP: sync::Lazy<RwLock<bool>> = sync::Lazy::new(|| RwLock::new(false));

#[node_bindgen]
fn start() {
    std::thread::spawn(move || loop {
        let sender = DEVICEMPSC.0.lock();
        let device_state = DeviceState::new();
        loop {
            let keys = device_state.get_keys();
            if *SHOULDSTOP.read() {
                return true;
            } else {
                let returnkeys: Vec<String> =
                    keys.clone().into_iter().map(|x| format!("{}", x)).collect();
                sender.send(returnkeys).unwrap();
            }
        }
    });
}

#[node_bindgen]
fn get_keys() -> Vec<String> {
    let reciever = DEVICEMPSC.1.lock();
    match reciever.recv() {
        Ok(s) => s,
        Err(e) => vec![e.to_string()],
    }
}

#[node_bindgen]
fn unload() {
    *SHOULDSTOP.write() = true;
}
