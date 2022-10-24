pub enum Key {
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    //Alt(char), control and alt are a pain to deal with
    //Ctrl(char),
    Esc,

    #[doc(hidden)]
    __IsNotComplete,
}

pub fn key_f(k: i32) -> Option<Key> {
    use Key::*;
    for i in 1..13 {
        if k == ncurses::KEY_F(i) {
            return Some(F(i));
        }
    }
    None
}

pub fn get_key() -> Key {
    use Key::*;
    let k = ncurses::getch();
    if k == -1 {
        panic!("There has been an error with ncurses please report this as a bug");
    } else if let Some(x) = key_f(k) {
        x
    } else {
        match ncurses::getch() {
            ncurses::KEY_BACKSPACE => Backspace,
            ncurses::KEY_LEFT => Left,
            ncurses::KEY_RIGHT => Right,
            ncurses::KEY_UP => Up,
            ncurses::KEY_DOWN => Down,
            ncurses::KEY_HOME => Home,
            ncurses::KEY_END => End,
            ncurses::KEY_PPAGE => PageUp,
            ncurses::KEY_NPAGE => PageDown,
            ncurses::KEY_BTAB => Tab,
            ncurses::KEY_DL => Delete,
            ncurses::KEY_IL => Insert,
            ncurses::KEY_EXIT => Esc,

            x => Char(char::from_u32(x as u32).unwrap()),
        }
    }
}
