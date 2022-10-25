pub use termion::event::Key;
pub enum SystemEvent {
    KeyPress(Key),
    WindowResize(u8, u8),
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
pub fn run<E: Event + std::marker::Send + 'static, M: Model<E>>(
    model: &mut M,
    cmd: Option<fn() -> E>,
) {
}
