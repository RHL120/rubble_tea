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
pub struct Spinner<E: crate::Event + Send + 'static> {
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

pub struct ProgressBar<E: crate::Event + Send + 'static> {
    add: E,
    take: E,
    color: crate::style::Color,
    n_elements: usize,
    idx: usize,
}

impl<E: crate::Event + Send + 'static> ProgressBar<E> {
    pub fn new(add: E, take: E, color: crate::style::Color, n_elements: usize) -> Self {
        ProgressBar {
            add,
            take,
            color,
            n_elements,
            idx: 0,
        }
    }
}
impl<E: crate::Event + Send + 'static> Widget<E> for ProgressBar<E> {
    fn init(&mut self) -> Option<Box<dyn FnOnce() -> E + Send + 'static>> {
        None
    }
    fn update(&mut self, e: &E) -> Option<Box<dyn FnOnce() -> E + Send + 'static>> {
        let e = e.clone();
        if e == self.add && self.idx < self.n_elements {
            self.idx += 1;
        } else if e == self.take && self.idx > 0 {
            self.idx -= 1;
        }
        None
    }
    fn view(&self) -> String {
        use crate::style::*;
        let bc = &self.color;
        format!(
            "{} {}%",
            (0..self.n_elements)
                .map(move |x| {
                    if x < self.idx {
                        StyleSheet::new()
                            .add(Style::TextColor(bc.clone()))
                            .render('█')
                    } else {
                        "░".to_string()
                    }
                })
                .fold(String::new(), |x, y| x + &y),
            self.idx * 100 / self.n_elements,
        )
    }
}

pub struct Timer<E: crate::Event + Send + 'static> {
    completed_event: E,
    pause_event: E,
    resume_event: E,
    update_event: E,
    paused: bool,
    pub time: f64,
}
impl<E: crate::Event + Send + 'static> Timer<E> {
    pub fn new(
        completed_event: E,
        pause_event: E,
        resume_event: E,
        update_event: E,
        time: f64,
    ) -> Self {
        Timer {
            completed_event,
            pause_event,
            resume_event,
            update_event,
            time,
            paused: false,
        }
    }
}
impl<E: crate::Event + Send + 'static> Widget<E> for Timer<E> {
    fn init(&mut self) -> Option<Box<dyn FnOnce() -> E + Send + 'static>> {
        let ns = self.update_event.clone();
        Some(Box::new(|| {
            std::thread::sleep(std::time::Duration::new(0, 10000000));
            ns
        }))
    }
    fn update(&mut self, e: &E) -> Option<Box<dyn FnOnce() -> E + Send + 'static>> {
        let e = e.clone();
        if e == self.update_event && self.time > 0.0 && !self.paused {
            self.time -= 0.01;
            Some(if self.time == 0.0 {
                let e = self.completed_event.clone();
                Box::new(move || e)
            } else {
                let e = self.update_event.clone();
                Box::new(move || {
                    std::thread::sleep(std::time::Duration::new(0, 10000000));
                    e
                })
            })
        } else if e == self.pause_event {
            self.paused = true;
            None
        } else if e == self.resume_event {
            let e = self.update_event.clone();
            Some(Box::new(move || e))
        } else {
            None
        }
    }
    fn view(&self) -> String {
        format!("{}", self.time)
    }
}
