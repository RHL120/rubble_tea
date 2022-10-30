///The trait that all widgets must implement
pub trait Widget<E: crate::Event + Send> {
    ///Sets up the widget and returns an initial event
    fn init(&mut self) -> Option<Box<dyn FnOnce() -> E + Send + 'static>>;
    ///Handles an event and returns another one
    fn update(&mut self, e: &E) -> Option<Box<dyn FnOnce() -> E + Send + 'static>>;
    ///Returns the string representation of a widget
    fn view(&self) -> String;
}

///The structure representation of a spinner
pub struct Spinner<E: crate::Event> {
    paused: bool,
    update_event: E,
    pause_event: E,
    resume_event: E,
    idx: usize,
}

impl<E: crate::Event + Send + 'static> Spinner<E> {
    pub fn new(update_event: E, pause_event: E, resume_event: E) -> Self {
        Spinner {
            update_event,
            pause_event,
            resume_event,
            paused: false,
            idx: 0,
        }
    }
}

const SPINNER_FRAMES: [&str; 8] = ["⣾ ", "⣽ ", "⣻ ", "⢿ ", "⡿ ", "⣟ ", "⣯ ", "⣷ "];

impl<E: crate::Event + Send + 'static> Widget<E> for Spinner<E> {
    fn init(&mut self) -> Option<Box<dyn FnOnce() -> E + Send + 'static>> {
        let e = self.resume_event.clone();
        Some(Box::new(move || e))
    }
    fn update(&mut self, e: &E) -> Option<Box<dyn FnOnce() -> E + Send + 'static>> {
        let e = e.clone();
        let update = self.update_event.clone();
        let updater = Box::new(|| {
            std::thread::sleep(std::time::Duration::new(0, 100000000));
            update
        });
        if self.update_event == e && !self.paused {
            self.idx = (self.idx + 1) % SPINNER_FRAMES.len();
            Some(updater)
        } else if self.resume_event == e {
            Some(updater)
        } else {
            if self.pause_event == e {
                self.paused = true;
            }
            None
        }
    }
    fn view(&self) -> String {
        SPINNER_FRAMES[self.idx].to_string()
    }
}
