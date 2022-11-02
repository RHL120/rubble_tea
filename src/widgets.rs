///The trait that all widgets must implement
pub trait Widget<E: crate::Event + Send> {
    ///Sets up the widget and returns the initial events
    fn init(&mut self) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>>;
    ///Handles an event and returns a set of events
    fn update(&mut self, e: &E) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>>;
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
    //Create a new spinner structure
    //*update_event* is the event the will be sent and received on updates
    //*pause_event* is the event the will be received on pause
    //*resume_event* is the event the will be received on resume
    pub fn new(update_event: E, pause_event: E, resume_event: E) -> Self {
        Spinner {
            update_event,
            pause_event,
            resume_event,
            paused: true,
            idx: 0,
        }
    }
}

const SPINNER_FRAMES: [&str; 8] = ["⣾ ", "⣽ ", "⣻ ", "⢿ ", "⡿ ", "⣟ ", "⣯ ", "⣷ "];

impl<E: crate::Event + Send + 'static> Widget<E> for Spinner<E> {
    fn init(&mut self) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        let e = self.resume_event.clone();
        vec![Box::new(move || e)]
    }
    fn update(&mut self, e: &E) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        let e = e.clone();
        let update = self.update_event.clone();
        let updater = Box::new(|| {
            std::thread::sleep(std::time::Duration::new(0, 100000000));
            update
        });
        if self.update_event == e && !self.paused {
            self.idx = (self.idx + 1) % SPINNER_FRAMES.len();
            vec![updater]
        } else if self.resume_event == e && self.paused {
            self.paused = false;
            vec![updater]
        } else {
            if self.pause_event == e {
                self.paused = true;
            }
            vec![]
        }
    }
    fn view(&self) -> String {
        SPINNER_FRAMES[self.idx].to_string()
    }
}

///The structure representation of a progress bar
pub struct ProgressBar<E: crate::Event + Send + 'static> {
    add: E,
    take: E,
    color: crate::style::Color,
    n_elements: usize,
    idx: usize,
}

impl<E: crate::Event + Send + 'static> ProgressBar<E> {
    ///Crates a new progress bar with
    ///*add* being the event received when the progress bar increments
    ///*take* being the event received when the progress bar decrements
    ///*n_elements* being the number of elements to be processed.
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
    fn init(&mut self) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        vec![]
    }
    fn update(&mut self, e: &E) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        let e = e.clone();
        if e == self.add && self.idx < self.n_elements {
            self.idx += 1;
        } else if e == self.take && self.idx > 0 {
            self.idx -= 1;
        }
        vec![]
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

///The structure representation of a timer
pub struct Timer<E: crate::Event + Send + 'static> {
    completed_event: E,
    pause_event: E,
    resume_event: E,
    update_event: E,
    paused: bool,
    ///The amount of time in seconds left for the timer to finish
    pub time: f32,
}
impl<E: crate::Event + Send + 'static> Timer<E> {
    ///Create a new timer with
    ///*completed_event* being the event to emmit when the timer is finished
    ///*pause_event* being the event that the timer receives to pause
    ///*resume_event* being the event that the timer receives to resume
    ///*update_event* being the event that the timer receives to update
    ///*time* the amount of time in seconds for the timer to finish
    pub fn new(
        completed_event: E,
        pause_event: E,
        resume_event: E,
        update_event: E,
        time: f32,
    ) -> Self {
        let time = (time * 100.0).round() / 100.0;
        Timer {
            completed_event,
            pause_event,
            resume_event,
            update_event,
            time,
            paused: true,
        }
    }
}
impl<E: crate::Event + Send + 'static> Widget<E> for Timer<E> {
    fn init(&mut self) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        let ns = self.resume_event.clone();
        vec![Box::new(|| {
            std::thread::sleep(std::time::Duration::new(0, 10000000));
            ns
        })]
    }
    fn update(&mut self, e: &E) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        let e = e.clone();
        if e == self.update_event && self.time > 0.0 && !self.paused {
            self.time = ((self.time - 0.01) * 100.0).round() / 100.0;
            vec![if self.time == 0.0 {
                let e = self.completed_event.clone();
                Box::new(move || e)
            } else {
                let e = self.update_event.clone();
                Box::new(move || {
                    std::thread::sleep(std::time::Duration::new(0, 10000000));
                    e
                })
            }]
        } else if e == self.pause_event {
            self.paused = true;
            vec![]
        } else if e == self.resume_event && self.paused {
            self.paused = false;
            let e = self.update_event.clone();
            vec![Box::new(move || e)]
        } else {
            vec![Box::new(move || e)]
        }
    }
    fn view(&self) -> String {
        format!("{:.2}", self.time)
    }
}

//The structure representation of a stop watch
pub struct StopWatch<E: crate::Event + Send + 'static> {
    resume_event: E,
    pause_event: E,
    update_event: E,
    paused: bool,
    ///The amount of seconds the stopwatch has been running
    pub time: f32,
}

impl<E: crate::Event + Send + 'static> StopWatch<E> {
    ///Creates a new stop watch with
    ///*pause_event* being the event that pauses the stopwatch
    ///*resume_event* being the event that resumes the stopwatch
    ///*update_event* being the event that updates the stopwatch
    pub fn new(pause_event: E, resume_event: E, update_event: E) -> Self {
        StopWatch {
            pause_event,
            resume_event,
            update_event,
            time: 0.0,
            paused: true,
        }
    }
}

impl<E: crate::Event + Send + 'static> Widget<E> for StopWatch<E> {
    fn init(&mut self) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        let ns = self.resume_event.clone();
        vec![Box::new(|| ns)]
    }
    fn update(&mut self, e: &E) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        let e = e.clone();
        if e == self.update_event && !self.paused {
            self.time = ((self.time + 0.01) * 100.0).round() / 100.0;
            vec![Box::new(|| {
                std::thread::sleep(std::time::Duration::new(0, 10000000));
                e
            })]
        } else if e == self.pause_event {
            self.paused = true;
            vec![]
        } else if e == self.resume_event && self.paused {
            let e = self.update_event.clone();
            self.paused = false;
            vec![Box::new(move || e)]
        } else {
            vec![]
        }
    }
    fn view(&self) -> String {
        format!("{:.2}", self.time)
    }
}

pub struct List<E: crate::Event + Send + 'static> {
    elements: Vec<String>,
    pages: Vec<(usize, usize)>,
    index_in_page: usize,
    page_index: usize,
    selected_style: Option<Box<dyn Fn(&str) -> String>>,
    unselected_style: Option<Box<dyn Fn(&str) -> String>>,
    up_event: E,
    down_event: E,
    chose_events: Option<(E, E)>,
    height: usize,
}
impl<E: crate::Event + Send + 'static> List<E> {
    pub fn new(
        elements: Vec<String>,
        up_event: E,
        down_event: E,
        chose_events: Option<(E, E)>,
        selected_style: Option<Box<dyn Fn(&str) -> String>>,
        unselected_style: Option<Box<dyn Fn(&str) -> String>>,
        height: u16,
    ) -> Self {
        let height = height as usize;
        let n_pages = (elements.len() as f32 / height as f32).ceil() as usize;
        let pages = (0..n_pages)
            .map(|x| (x * height, x * height + height))
            .collect();
        List {
            elements,
            pages,
            index_in_page: 0,
            page_index: 0,
            selected_style,
            unselected_style,
            up_event,
            down_event,
            height,
            chose_events,
        }
    }
}
impl<E: crate::Event + Send + 'static> Widget<E> for List<E> {
    fn init(&mut self) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        vec![]
    }
    fn update(&mut self, e: &E) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        let e = e.clone();
        if self.down_event == e {
            let new_index_in_page = self.index_in_page + 1;
            if new_index_in_page < self.height {
                self.index_in_page = new_index_in_page;
            } else {
                let new_page_index = self.page_index + 1;
                if new_page_index < self.pages.len() {
                    self.page_index = new_page_index;
                    self.index_in_page = 0;
                }
            }
        } else if self.up_event == e {
            if let Some(new_index_in_page) = self.index_in_page.checked_sub(1) {
                self.index_in_page = new_index_in_page;
            } else {
                if let Some(new_page_index) = self.page_index.checked_sub(1) {
                    self.page_index = new_page_index;
                    self.index_in_page = self.height - 1;
                }
            }
        } else if let Some((r, s)) = &self.chose_events {
            let r = r.clone();
            if e == r {
                let s = s.clone();
                return vec![Box::new(|| s)];
            }
        }
        vec![]
    }
    fn view(&self) -> String {
        let mut ret = String::new();
        let first = self.pages[self.page_index].0;
        for i in first..self.pages[self.page_index].1 {
            if i != first {
                ret += "\n\r"
            }
            if i - first == self.index_in_page {
                if let Some(f) = &self.selected_style {
                    ret += &f(&self.elements[i]);
                } else {
                    ret += &format!("*>{}", self.elements[i]);
                }
            } else {
                if let Some(f) = &self.unselected_style {
                    ret += &f(&self.elements[i]);
                } else {
                    ret += &self.elements[i];
                }
            }
            println!("{}", self.elements[i]);
        }
        ret
    }
}
