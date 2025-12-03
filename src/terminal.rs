pub enum HighlightColor {
    Red,
    Green,
    Blue,
    Yellow,
    None,
}

impl From<&str> for HighlightColor {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "red" => HighlightColor::Red,
            "green" => HighlightColor::Green,
            "blue" => HighlightColor::Blue,
            "yellow" => HighlightColor::Yellow,
            "none" => HighlightColor::None,
            _ => HighlightColor::Red,
        }
    }
}

pub fn highlight_to_code(color: &HighlightColor) -> &'static str {
    match color {
        HighlightColor::Red => "\x1b[31m",
        HighlightColor::Green => "\x1b[32m",
        HighlightColor::Blue => "\x1b[34m",
        HighlightColor::Yellow => "\x1b[33m",
        HighlightColor::None => "",
    }
}

pub fn print_error(msg: &str) {
    eprintln!("\x1b[31m\x1b[1mError\x1b[0m: {msg}")
}
