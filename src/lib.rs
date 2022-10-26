pub mod style;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub use termion::event::Key;
#[derive(Clone)]
///System events are the set of events that all models should support
pub enum SystemEvent {
    ///When a key has been pressed
    KeyPress(Key),
    ///When the window has been resized
    WindowResize(u16, u16),
    ///This causes the main loop to break, usually emmited from update.
    Quit,
}

///This trait allows the user to create custom events.
///*SystemEvent* implements this trait meaning that if the programer is content
///with the default events, they don't have to create their own wrapper.
pub trait Event {
    fn from_system_event(se: SystemEvent) -> Self;
    fn to_system_event(&self) -> Option<SystemEvent>;
}

pub trait Model<E: Event> {
    fn update(&mut self, e: &E) -> Option<fn() -> E>;
    fn view(&self) -> String;
}

impl Event for SystemEvent {
    fn from_system_event(se: SystemEvent) -> Self {
        se
    }
    fn to_system_event(&self) -> Option<SystemEvent> {
        Some(self.clone())
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
            tx.send(E::from_system_event(SystemEvent::WindowResize(nw, nh)))
                .unwrap();
            (ow, oh) = (ow, oh)
        }
    }
}

///Starts the event listeners and the main program loop
pub fn run<E: Event + std::marker::Send + 'static, M: Model<E>>(
    model: &mut M,
    cmd: Option<fn() -> E>,
) {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let (tx, rx): (mpsc::Sender<E>, mpsc::Receiver<E>) = mpsc::channel();
    {
        let tx = tx.clone();
        std::thread::spawn(move || watch_keys(tx));
    }
    {
        let tx = tx.clone();
        std::thread::spawn(move || watch_resize(tx));
    }
    if let Some(f) = cmd {
        let tx = tx.clone();
        std::thread::spawn(move || tx.send(f()));
    }
    //We are guaranteed to recive at least one event on startup (the resize event)
    for i in rx.iter() {
        if let Some(f) = model.update(&i) {
            let tx = tx.clone();
            std::thread::spawn(move || tx.send(f()));
        }
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            model.view()
        )
        .unwrap();
        stdout.flush().unwrap();
        if let Some(x) = i.to_system_event() {
            match x {
                SystemEvent::Quit => break,
                _ => (),
            }
        }
    }
}
