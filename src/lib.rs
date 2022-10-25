use std::io::{stdin, stdout, Write};
use std::sync::mpsc;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub use termion::event::Key;
pub enum SystemEvent {
    KeyPress(Key),
    WindowResize(u16, u16),
}

pub trait Event {
    fn from_system_event(se: SystemEvent) -> Self;
}

pub trait Model<E: Event> {
    fn update(&mut self, e: E) -> Option<fn() -> E>;
    fn view(&self) -> String;
}

impl Event for SystemEvent {
    fn from_system_event(se: SystemEvent) -> Self {
        se
    }
}

fn watch_keys<E: Event>(tx: mpsc::Sender<E>) {
    let stdin = stdin();
    for i in stdin.keys() {
        tx.send(E::from_system_event(SystemEvent::KeyPress(i.unwrap())))
            .unwrap();
    }
}

fn watch_resize<E: Event>(tx: mpsc::Sender<E>) {
    let (mut ow, mut oh) = termion::terminal_size().unwrap();
    tx.send(E::from_system_event(SystemEvent::WindowResize(ow, oh)))
        .unwrap();
    loop {
        std::thread::sleep(std::time::Duration::new(0, 500000000));
        let (nw, nh) = termion::terminal_size().unwrap();
        if nw != ow || nh != oh {
            tx.send(E::from_system_event(SystemEvent::WindowResize(ow, oh)))
                .unwrap();
            (ow, oh) = (ow, oh)
        }
    }
}
