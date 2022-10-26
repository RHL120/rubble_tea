#[derive(Eq, PartialEq)]
pub enum TextStyle {
    Bold,
    Faint,
    Italic,
    Underline,
    Blink,
    Invert,
    StrikeThrough,
}

#[derive(Eq, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite,
}

#[derive(Eq, PartialEq)]
pub enum Position {
    Absolute(u16, u16),
    Center,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Eq, PartialEq)]
pub enum Border {
    Top,
    Left,
    Right,
    Bottom,
}

#[derive(Eq, PartialEq)]
pub enum Style {
    TextStyle(TextStyle),
    TextColor(Color),
    BackgroundColor(Color),
    Position(Position),
    Border(Border),
}

pub struct StyleSheet(Vec<Style>);

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Style::TextStyle(s) => match s {
                TextStyle::Bold => write!(f, "{}", termion::style::Bold),
                TextStyle::Faint => write!(f, "{}", termion::style::Faint),
                TextStyle::Italic => write!(f, "{}", termion::style::Italic),
                TextStyle::Underline => write!(f, "{}", termion::style::Underline),
                TextStyle::Blink => write!(f, "{}", termion::style::Blink),
                TextStyle::Invert => write!(f, "{}", termion::style::Invert),
                TextStyle::StrikeThrough => write!(f, "{}", termion::style::CrossedOut),
            },
            Style::TextColor(c) => match c {
                Color::Black => write!(f, "{}", termion::color::Fg(termion::color::Black)),
                Color::Red => write!(f, "{}", termion::color::Fg(termion::color::Red)),
                Color::Green => write!(f, "{}", termion::color::Fg(termion::color::Green)),
                Color::Yellow => write!(f, "{}", termion::color::Fg(termion::color::Yellow)),
                Color::Blue => write!(f, "{}", termion::color::Fg(termion::color::Blue)),
                Color::Magenta => write!(f, "{}", termion::color::Fg(termion::color::Magenta)),
                Color::Cyan => write!(f, "{}", termion::color::Fg(termion::color::Cyan)),
                Color::White => write!(f, "{}", termion::color::Fg(termion::color::White)),
                Color::LightBlack => {
                    write!(f, "{}", termion::color::Fg(termion::color::LightBlack))
                }
                Color::LightRed => write!(f, "{}", termion::color::Fg(termion::color::LightRed)),
                Color::LightGreen => {
                    write!(f, "{}", termion::color::Fg(termion::color::LightGreen))
                }
                Color::LightYellow => {
                    write!(f, "{}", termion::color::Fg(termion::color::LightYellow))
                }
                Color::LightBlue => write!(f, "{}", termion::color::Fg(termion::color::LightBlue)),
                Color::LightMagenta => {
                    write!(f, "{}", termion::color::Fg(termion::color::LightMagenta))
                }
                Color::LightCyan => write!(f, "{}", termion::color::Fg(termion::color::LightCyan)),
                Color::LightWhite => {
                    write!(f, "{}", termion::color::Fg(termion::color::LightWhite))
                }
            },
            Style::BackgroundColor(c) => match c {
                Color::Black => write!(f, "{}", termion::color::Bg(termion::color::Black)),
                Color::Red => write!(f, "{}", termion::color::Bg(termion::color::Red)),
                Color::Green => write!(f, "{}", termion::color::Bg(termion::color::Green)),
                Color::Yellow => write!(f, "{}", termion::color::Bg(termion::color::Yellow)),
                Color::Blue => write!(f, "{}", termion::color::Bg(termion::color::Blue)),
                Color::Magenta => write!(f, "{}", termion::color::Bg(termion::color::Magenta)),
                Color::Cyan => write!(f, "{}", termion::color::Bg(termion::color::Cyan)),
                Color::White => write!(f, "{}", termion::color::Bg(termion::color::White)),
                Color::LightBlack => {
                    write!(f, "{}", termion::color::Bg(termion::color::LightBlack))
                }
                Color::LightRed => write!(f, "{}", termion::color::Bg(termion::color::LightRed)),
                Color::LightGreen => {
                    write!(f, "{}", termion::color::Bg(termion::color::LightGreen))
                }
                Color::LightYellow => {
                    write!(f, "{}", termion::color::Bg(termion::color::LightYellow))
                }
                Color::LightBlue => write!(f, "{}", termion::color::Bg(termion::color::LightBlue)),
                Color::LightMagenta => {
                    write!(f, "{}", termion::color::Bg(termion::color::LightMagenta))
                }
                Color::LightCyan => write!(f, "{}", termion::color::Bg(termion::color::LightCyan)),
                Color::LightWhite => {
                    write!(f, "{}", termion::color::Bg(termion::color::LightWhite))
                }
            },
            Style::Position(pos) => match pos {
                Position::Absolute(x, y) => write!(f, "{}", termion::cursor::Goto(x + 1, y + 1)),
                Position::TopLeft => write!(f, "{}", termion::cursor::Goto(1, 1)),
                Position::Center => {
                    let (w, h) = termion::terminal_size().unwrap();
                    write!(f, "{}", termion::cursor::Goto(w / 2, h / 2))
                }
                Position::TopRight => {
                    let (w, _) = termion::terminal_size().unwrap();
                    write!(f, "{}", termion::cursor::Goto(w, 1))
                }
                Position::BottomLeft => {
                    let (_, h) = termion::terminal_size().unwrap();
                    write!(f, "{}", termion::cursor::Goto(1, h))
                }
                Position::BottomRight => {
                    let (w, h) = termion::terminal_size().unwrap();
                    write!(f, "{}", termion::cursor::Goto(w, h))
                }
            },
            Style::Border(_) => todo!(),
        }
    }
}

impl StyleSheet {
    pub fn new() -> Self {
        StyleSheet(Vec::new())
    }
    pub fn add(mut self, s: Style) -> StyleSheet {
        self.0.push(s);
        self
    }
    pub fn render<D: std::fmt::Display>(StyleSheet(sheet): &Self, d: D) -> String {
        let mut end = String::new();
        let mut ret = String::new();
        for i in sheet {
            if let Style::TextStyle(_) = i {
                end = format!("{}", termion::style::Reset);
            } else if let Style::TextColor(_) = i {
                end = format!("{}", termion::style::Reset);
            } else if let Style::BackgroundColor(_) = i {
                end = format!("{}", termion::style::Reset);
            }
            ret = format!("{}{}", ret, i);
        }
        format!("{}{}{}", ret, d, end)
    }
}
