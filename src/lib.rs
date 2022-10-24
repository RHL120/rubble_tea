pub mod events;
use std::sync::mpsc;

pub trait Model<E: events::Event> {
    fn update(&mut self, e: E) -> Option<fn() -> E>;
    fn view(&self) -> String;
}

pub fn run<E: events::Event + std::marker::Send + 'static, M: Model<E>>(
    model: &mut M,
    cmd: Option<fn() -> E>,
) {
    ncurses::initscr();
    ncurses::raw();
    ncurses::keypad(ncurses::stdscr(), true);
    ncurses::noecho();
    let (tx, rx): (mpsc::Sender<E>, mpsc::Receiver<E>) = mpsc::channel();
    {
        let tx = tx.clone();
        std::thread::spawn(move || events::watch_key(tx));
    }
    {
        let tx = tx.clone();
        std::thread::spawn(move || events::watch_size_change(tx));
    }
    if let Some(f) = cmd {
        let tx = tx.clone();
        std::thread::spawn(move || tx.send(f()).unwrap());
    }
    loop {
        for i in rx.iter() {
            if let Some(f) = model.update(i) {
                let tx = tx.clone();
                std::thread::spawn(move || tx.send(f()).unwrap());
            }
            ncurses::clear();
            ncurses::addstr(model.view().as_str());
            ncurses::refresh();
        }
    }
}
