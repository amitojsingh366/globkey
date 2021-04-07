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

static DEVICETHREAD: sync::OnceCell<Mutex<Option<std::thread::JoinHandle<bool>>>> =
    sync::OnceCell::new();

#[node_bindgen]
fn start() {
    *SHOULDSTOP.write() = false;
    DEVICETHREAD
        .set(Mutex::new(Some(std::thread::spawn(move || loop {
            let sender = DEVICEMPSC.0.lock();
            let device_state = DeviceState::new();
            let mut prev_keys = vec![];
            loop {
                let keys = device_state.get_keys();
                if *SHOULDSTOP.read() {
                    return true;
                } else if keys != prev_keys {
                    let returnkeys: Vec<String> =
                        keys.clone().into_iter().map(|x| format!("{}", x)).collect();
                    sender.send(returnkeys).unwrap();
                }
                prev_keys = keys;
            }
        }))))
        .unwrap();
}

#[node_bindgen]
fn get_keys() -> Result<Vec<String>, bool> {
    let reciever = DEVICEMPSC.1.lock();
    match *SHOULDSTOP.read() {
        true => Err(false),
        _ => match DEVICETHREAD.get() {
            Some(_) => match reciever.recv() {
                Ok(s) => Ok(s),
                Err(_) => Err(false),
            },
            None => Err(false),
        },
    }
}

#[node_bindgen]
fn unload() {
    *SHOULDSTOP.write() = true;
}
