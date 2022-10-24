pub use super::backend::Event;
pub use super::backend::Key;
pub use super::backend::SystemEvent;
use super::backend::*;
use std::sync::mpsc;
pub fn watch_key(tx: mpsc::Sender<SystemEvent>) {
    loop {
        let key = get_key();
        match tx.send(SystemEvent::KeyPress(key)) {
            Ok(_) => (),
            Err(_) => panic!("Communication error"),
        }
    }
}
pub fn watch_size_change(tx: mpsc::Sender<SystemEvent>) {
    let (mut ow, mut oh) = get_window_size();
    loop {
        std::thread::sleep(std::time::Duration::new(0, 500000000));
        let (nw, nh) = get_window_size();
        if nw != ow || nh != oh {
            match tx.send(SystemEvent::WindowResize(nw, nh)) {
                Ok(_) => (),
                Err(_) => panic!("Communication error"),
            }
            (ow, oh) = (nw, nh);
        }
    }
}
