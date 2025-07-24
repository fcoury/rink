use crossterm::style::Color;

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub dim: bool,
    pub strikethrough: bool,
}

impl Style {
    pub fn to_ansi_codes(&self) -> String {
        let mut codes = Vec::new();

        // Reset first to clear any previous styling
        codes.push("0".to_string());

        // Foreground color
        if let Some(color) = &self.fg {
            codes.push(color_to_ansi_fg(color));
        }

        // Background color
        if let Some(color) = &self.bg {
            codes.push(color_to_ansi_bg(color));
        }

        // Text attributes
        if self.bold {
            codes.push("1".to_string());
        }
        if self.dim {
            codes.push("2".to_string());
        }
        if self.italic {
            codes.push("3".to_string());
        }
        if self.underline {
            codes.push("4".to_string());
        }
        if self.strikethrough {
            codes.push("9".to_string());
        }

        format!("\x1b[{}m", codes.join(";"))
    }
}

fn color_to_ansi_fg(color: &Color) -> String {
    match color {
        Color::Black => "30".to_string(),
        Color::DarkGrey => "90".to_string(),
        Color::Red => "31".to_string(),
        Color::DarkRed => "31".to_string(),
        Color::Green => "32".to_string(),
        Color::DarkGreen => "32".to_string(),
        Color::Yellow => "33".to_string(),
        Color::DarkYellow => "33".to_string(),
        Color::Blue => "34".to_string(),
        Color::DarkBlue => "34".to_string(),
        Color::Magenta => "35".to_string(),
        Color::DarkMagenta => "35".to_string(),
        Color::Cyan => "36".to_string(),
        Color::DarkCyan => "36".to_string(),
        Color::White => "37".to_string(),
        Color::Grey => "37".to_string(),
        Color::Rgb { r, g, b } => format!("38;2;{};{};{}", r, g, b),
        Color::AnsiValue(value) => format!("38;5;{}", value),
        Color::Reset => "39".to_string(),
    }
}

fn color_to_ansi_bg(color: &Color) -> String {
    match color {
        Color::Black => "40".to_string(),
        Color::DarkGrey => "100".to_string(),
        Color::Red => "41".to_string(),
        Color::DarkRed => "41".to_string(),
        Color::Green => "42".to_string(),
        Color::DarkGreen => "42".to_string(),
        Color::Yellow => "43".to_string(),
        Color::DarkYellow => "43".to_string(),
        Color::Blue => "44".to_string(),
        Color::DarkBlue => "44".to_string(),
        Color::Magenta => "45".to_string(),
        Color::DarkMagenta => "45".to_string(),
        Color::Cyan => "46".to_string(),
        Color::DarkCyan => "46".to_string(),
        Color::White => "47".to_string(),
        Color::Grey => "47".to_string(),
        Color::Rgb { r, g, b } => format!("48;2;{};{};{}", r, g, b),
        Color::AnsiValue(value) => format!("48;5;{}", value),
        Color::Reset => "49".to_string(),
    }
}
