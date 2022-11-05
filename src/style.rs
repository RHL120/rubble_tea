#[derive(Eq, PartialEq, Clone)]
/// They way the text should be displayed
pub enum TextStyle {
    Bold,
    Faint,
    Italic,
    Underline,
    Blink,
    Invert,
    StrikeThrough,
}

#[derive(Eq, PartialEq, Clone)]
///The color to be printed
pub enum Color {
    ///The terminal's default black color
    Black,
    ///The terminal's default red color
    Red,
    ///The terminal's default green color
    Green,
    ///The terminal's default yello color
    Yellow,
    ///The terminal's default blue color
    Blue,
    ///The terminal's default magenta color
    Magenta,
    ///The terminal's default cyan color
    Cyan,
    ///The terminal's default white color
    White,
    ///The terminal's default light black color
    LightBlack,
    ///The terminal's default light red color
    LightRed,
    ///The terminal's default light green red color
    LightGreen,
    ///The terminal's default light yellow red color
    LightYellow,
    ///The terminal's default light blue red color
    LightBlue,
    ///The terminal's default light magenta red color
    LightMagenta,
    ///The terminal's default light cyan red color
    LightCyan,
    ///The terminal's default light white red color
    LightWhite,
}

#[derive(Eq, PartialEq, Clone)]
///The position of the text
pub enum Position {
    ///Zero indexed position
    Absolute(u16, u16),
    ///Start the text at the center of the screen
    Center,
    ///Start the text at the top left of the screen
    TopLeft,
    ///Start the text at the top right of the screen
    TopRight,
    ///Start the text at the bottom left of the screen
    BottomLeft,
    ///Start the text at the bottom right of the screen
    BottomRight,
}

#[derive(Eq, PartialEq, Clone)]
///Combination of all styles structures
pub enum Style {
    /// The style of the text
    TextStyle(TextStyle),
    /// The color of the text
    TextColor(Color),
    /// The background color
    BackgroundColor(Color),
    /// The position of the text
    Position(Position),
}

/// A list of styles
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
        }
    }
}

impl StyleSheet {
    /// Create an empty stylesheet
    pub fn new() -> Self {
        StyleSheet(Vec::new())
    }
    /// Add another style to the style sheet
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, s: Style) -> StyleSheet {
        self.0.push(s);
        self
    }
    #[warn(clippy::should_implement_trait)]
    /// Apply the given stylesheet onto the given object
    pub fn render<D: std::fmt::Display>(&self, d: D) -> String {
        let mut end = String::new();
        let mut ret = String::new();
        let StyleSheet(sheet) = self;
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

impl Default for StyleSheet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn style_bold() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextStyle(TextStyle::Bold))
                .render("hello"),
            "\u{1b}[1mhello\u{1b}[m"
        );
    }
    #[test]
    fn style_faint() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextStyle(TextStyle::Faint))
                .render("hello"),
            "\u{1b}[2mhello\u{1b}[m"
        );
    }
    #[test]
    fn style_italic() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextStyle(TextStyle::Italic))
                .render("hello"),
            "\u{1b}[3mhello\u{1b}[m"
        );
    }
    #[test]
    fn style_underline() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextStyle(TextStyle::Underline))
                .render("hello"),
            "\u{1b}[4mhello\u{1b}[m"
        );
    }
    #[test]
    fn style_blink() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextStyle(TextStyle::Blink))
                .render("hello"),
            "\u{1b}[5mhello\u{1b}[m"
        );
    }
    #[test]
    fn style_invert() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextStyle(TextStyle::Invert))
                .render("hello"),
            "\u{1b}[7mhello\u{1b}[m"
        );
    }
    #[test]
    fn style_strike_through() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextStyle(TextStyle::Invert))
                .render("hello"),
            "\u{1b}[7mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_black() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::Black))
                .render("hello"),
            "\u{1b}[48;5;0mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_red() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::Red))
                .render("hello"),
            "\u{1b}[48;5;1mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_green() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::Green))
                .render("hello"),
            "\u{1b}[48;5;2mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_yellow() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::Yellow))
                .render("hello"),
            "\u{1b}[48;5;3mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_blue() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::Blue))
                .render("hello"),
            "\u{1b}[48;5;4mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_magenta() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::Magenta))
                .render("hello"),
            "\u{1b}[48;5;5mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_cyan() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::Cyan))
                .render("hello"),
            "\u{1b}[48;5;6mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_white() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::White))
                .render("hello"),
            "\u{1b}[48;5;7mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_light_black() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::LightBlack))
                .render("hello"),
            "\u{1b}[48;5;8mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_light_red() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::LightRed))
                .render("hello"),
            "\u{1b}[48;5;9mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_light_green() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::LightGreen))
                .render("hello"),
            "\u{1b}[48;5;10mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_light_yellow() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::LightYellow))
                .render("hello"),
            "\u{1b}[48;5;11mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_light_blue() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::LightBlue))
                .render("hello"),
            "\u{1b}[48;5;12mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_light_magenta() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::LightMagenta))
                .render("hello"),
            "\u{1b}[48;5;13mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_light_cyan() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::LightCyan))
                .render("hello"),
            "\u{1b}[48;5;14mhello\u{1b}[m"
        );
    }
    #[test]
    fn bg_color_light_white() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::BackgroundColor(Color::LightWhite))
                .render("hello"),
            "\u{1b}[48;5;15mhello\u{1b}[m"
        );
    }

    #[test]
    fn fg_color_black() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::Black))
                .render("hello"),
            "\u{1b}[38;5;0mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_red() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::Red))
                .render("hello"),
            "\u{1b}[38;5;1mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_green() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::Green))
                .render("hello"),
            "\u{1b}[38;5;2mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_yellow() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::Yellow))
                .render("hello"),
            "\u{1b}[38;5;3mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_blue() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::Blue))
                .render("hello"),
            "\u{1b}[38;5;4mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_magenta() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::Magenta))
                .render("hello"),
            "\u{1b}[38;5;5mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_cyan() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::Cyan))
                .render("hello"),
            "\u{1b}[38;5;6mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_white() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::White))
                .render("hello"),
            "\u{1b}[38;5;7mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_light_black() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::LightBlack))
                .render("hello"),
            "\u{1b}[38;5;8mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_light_red() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::LightRed))
                .render("hello"),
            "\u{1b}[38;5;9mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_light_green() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::LightGreen))
                .render("hello"),
            "\u{1b}[38;5;10mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_light_yellow() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::LightYellow))
                .render("hello"),
            "\u{1b}[38;5;11mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_light_blue() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::LightBlue))
                .render("hello"),
            "\u{1b}[38;5;12mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_light_magenta() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::LightMagenta))
                .render("hello"),
            "\u{1b}[38;5;13mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_light_cyan() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::LightCyan))
                .render("hello"),
            "\u{1b}[38;5;14mhello\u{1b}[m"
        );
    }
    #[test]
    fn fg_color_light_white() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::TextColor(Color::LightWhite))
                .render("hello"),
            "\u{1b}[38;5;15mhello\u{1b}[m"
        );
    }

    #[test]
    fn position_abs() {
        assert_eq!(
            StyleSheet::new()
                .add(Style::Position(Position::Absolute(5, 5)))
                .render("hello"),
            "\u{1b}[6;6Hhello"
        );
    }
}
