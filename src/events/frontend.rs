pub use super::backend::Key;
use super::backend::*;
use std::sync::mpsc;

pub enum SystemEvent {
    KeyPress(Key),
    WindowResize(u8, u8),
}

pub trait Event {
    fn from_system_event(se: SystemEvent) -> Self;
}

impl Event for SystemEvent {
    fn from_system_event(se: SystemEvent) -> Self {
        se
    }
}

pub fn watch_key<E: Event>(tx: mpsc::Sender<E>) {
    loop {
        let key = get_key();
        match tx.send(E::from_system_event(SystemEvent::KeyPress(key))) {
            Ok(_) => (),
            Err(_) => panic!("Communication error"),
        }
    }
}
pub fn watch_size_change<E: Event>(tx: mpsc::Sender<E>) {
    let (mut ow, mut oh) = get_window_size();
    loop {
        std::thread::sleep(std::time::Duration::new(0, 500000000));
        let (nw, nh) = get_window_size();
        if nw != ow || nh != oh {
            match tx.send(E::from_system_event(SystemEvent::WindowResize(nw, nh))) {
                Ok(_) => (),
                Err(_) => panic!("Communication error"),
            }
            (ow, oh) = (nw, nh);
        }
    }
}
