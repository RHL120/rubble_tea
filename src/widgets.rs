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

//The structure representation of a List
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
    ///Creates a new list with
    ///*elements* being the elements in the list
    ///*up_event* being the event for moving the cursor up
    ///*down_event* being the event for moving the cursor down
    ///*chose_events* the first event is the event that when recived, indicates
    ///that the selected element has been chosen, the second is the event to be
    ///emmited when an element has been chosen.
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

///The structure representation of a text-input
pub struct TextInput<E: crate::Event + Send + 'static> {
    input: String,
    cursor_index: usize,
    submission_event: E,
    toggle_blink_event: Option<E>,
    blinks: bool,
}
impl<E: crate::Event + Send + 'static> TextInput<E> {
    ///Create a new TextInput with
    ///*submission_event* being the event sent when enter is pressed
    ///*toggle_blink_event* the event that when received turns off the cursor if
    ///it is on and turns it on if it is off
    pub fn new(submission_event: E, toggle_blink_event: Option<E>) -> Self {
        TextInput {
            input: String::new(),
            cursor_index: 0,
            submission_event,
            toggle_blink_event,
            blinks: true,
        }
    }
    ///Get the input string
    pub fn get_string<'a>(&'a self) -> &'a str {
        &self.input
    }
    ///Clear the input string
    pub fn clear_string(&mut self) {
        self.input = String::new();
    }
    ///Is the cursor blinking
    pub fn is_blinking(&self) -> bool {
        return self.blinks;
    }
}

impl<E: crate::Event + Send + 'static> Widget<E> for TextInput<E> {
    fn init(&mut self) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        vec![]
    }
    fn update(&mut self, e: &E) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        use crate::Key::*;
        use crate::SystemEvent::*;
        if let Some(be) = &self.toggle_blink_event {
            let e = e.clone();
            let be = be.clone();
            if be == e {
                self.blinks = !self.blinks;
            }
        }
        if let Some(event) = e.to_system_event() {
            match event {
                KeyPress(Left) => {
                    if self.cursor_index > 0 {
                        self.cursor_index -= 1;
                    }
                }
                KeyPress(Right) => {
                    if self.cursor_index < self.input.len() {
                        self.cursor_index += 1;
                    }
                }
                KeyPress(Char('\n')) => {
                    let e = self.submission_event.clone();
                    return vec![Box::new(|| e)];
                }
                KeyPress(Char(x)) => {
                    self.input = format!(
                        "{}{}{}",
                        &self.input[..self.cursor_index],
                        x,
                        &self.input[self.cursor_index..],
                    );
                    self.cursor_index += 1
                }
                KeyPress(Backspace) => {
                    if self.cursor_index > 0 {
                        self.input = format!(
                            "{}{}",
                            &self.input[..self.cursor_index - 1],
                            &self.input[self.cursor_index..],
                        );
                        self.cursor_index -= 1
                    }
                }
                _ => (),
            };
        }
        return vec![];
    }
    fn view(&self) -> String {
        use crate::style::*;
        let cursor = if self.blinks {
            StyleSheet::new()
                .add(Style::TextStyle(TextStyle::Blink))
                .render("|")
        } else {
            String::from("|")
        };
        format!(
            "{}{}{}",
            &self.input[..self.cursor_index],
            cursor,
            &self.input[self.cursor_index..]
        )
    }
}

///The structure representation
pub struct ViewPort<E: crate::Event + Send + 'static> {
    up_event: E,
    down_event: E,
    start_line: usize,
    ///The contents of the string
    pub string: String,
    ///The height of the viewport
    pub height: u16,
    ///The width of the viewport
    pub width: u16,
}
impl<E: crate::Event + Send + 'static> ViewPort<E> {
    ///Creates a new viewport with
    ///*string* being the contents of the viewport
    ///*up_event* being the event to scroll up
    ///*down_event* being the event to scroll down
    ///*width* being the width of the viewport
    ///*height* is the height of the viewport
    pub fn new(string: String, up_event: E, down_event: E, width: u16, height: u16) -> Self {
        ViewPort {
            up_event,
            down_event,
            string,
            width,
            height,
            start_line: 0,
        }
    }
    //The way this function is implemented and the way this whole widget is implemented
    //is painfully slow.
    fn split_lines(&self) -> Vec<String> {
        let mut res = Vec::new();
        let mut char_idx: usize = 0;
        let mut line = String::new();
        let mut skip_nline = false;
        for chr in self.string.chars() {
            if chr == '\n' {
                if skip_nline {
                    skip_nline = false;
                    continue;
                }
                char_idx = 0;
                res.push(line);
                line = String::new();
            } else if char_idx >= self.width as usize - 1 {
                char_idx = 0;
                line += &chr.to_string();
                res.push(line);
                line = String::new();
                skip_nline = true;
            } else {
                line += &chr.to_string();
                char_idx += 1;
            }
        }
        if line != "" {
            res.push(line);
        }
        res
    }
}

impl<E: crate::Event + Send + 'static> Widget<E> for ViewPort<E> {
    fn init(&mut self) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        vec![]
    }
    fn update(&mut self, e: &E) -> Vec<Box<dyn FnOnce() -> E + Send + 'static>> {
        let e = e.clone();
        if e == self.up_event {
            self.start_line = self.start_line.checked_sub(1).unwrap_or(self.start_line);
        } else if e == self.down_event {
            //I should probably check if the new start_line is in the range of the
            //number of lines here but this is hard to do since the actual
            //lining is done by view
            self.start_line = self.start_line.checked_add(1).unwrap_or(self.start_line);
        }
        vec![]
    }
    fn view(&self) -> String {
        let ret = self.split_lines();
        let end = self.height as usize + self.start_line;
        let end = if end > ret.len() { ret.len() } else { end };
        ret[self.start_line..end].join("\r\n")
    }
}
