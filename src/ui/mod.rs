use std::io::Write;

use termion::color;

use self::turn_bar::TurnBar;

pub mod boxx;
pub mod turn_bar;

pub struct UITheme {
    pub turn_bar: TurnBar,
}

pub enum EColor {
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
impl EColor {
    pub fn write_fg<W: Write>(self, op: &mut W) -> std::io::Result<()>{
        match self {
            EColor::Black => write!(op, "{}", color::Fg(color::Black)),
            EColor::Red => write!(op, "{}", color::Fg(color::Red)),
            EColor::Green => write!(op, "{}", color::Fg(color::Green)),
            EColor::Yellow => write!(op, "{}", color::Fg(color::Yellow)),
            EColor::Blue => write!(op, "{}", color::Fg(color::Blue)),
            EColor::Magenta => write!(op, "{}", color::Fg(color::Magenta)),
            EColor::Cyan => write!(op, "{}", color::Fg(color::Cyan)),
            EColor::White => write!(op, "{}", color::Fg(color::White)),
            EColor::LightBlack => write!(op, "{}", color::Fg(color::LightBlack)),
            EColor::LightRed => write!(op, "{}", color::Fg(color::LightRed)),
            EColor::LightGreen => write!(op, "{}", color::Fg(color::LightGreen)),
            EColor::LightYellow => write!(op, "{}", color::Fg(color::LightYellow)),
            EColor::LightBlue => write!(op, "{}", color::Fg(color::LightBlue)),
            EColor::LightMagenta => write!(op, "{}", color::Fg(color::LightMagenta)),
            EColor::LightCyan => write!(op, "{}", color::Fg(color::LightCyan)),
            EColor::LightWhite => write!(op, "{}", color::Fg(color::LightWhite)),
        }
    }
    pub fn write_bg<W: Write>(self, op: &mut W) -> std::io::Result<()>{
        match self {
            EColor::Black => write!(op, "{}", color::Bg(color::Black)),
            EColor::Red => write!(op, "{}", color::Bg(color::Red)),
            EColor::Green => write!(op, "{}", color::Bg(color::Green)),
            EColor::Yellow => write!(op, "{}", color::Bg(color::Yellow)),
            EColor::Blue => write!(op, "{}", color::Bg(color::Blue)),
            EColor::Magenta => write!(op, "{}", color::Bg(color::Magenta)),
            EColor::Cyan => write!(op, "{}", color::Bg(color::Cyan)),
            EColor::White => write!(op, "{}", color::Bg(color::White)),
            EColor::LightBlack => write!(op, "{}", color::Bg(color::LightBlack)),
            EColor::LightRed => write!(op, "{}", color::Bg(color::LightRed)),
            EColor::LightGreen => write!(op, "{}", color::Bg(color::LightGreen)),
            EColor::LightYellow => write!(op, "{}", color::Bg(color::LightYellow)),
            EColor::LightBlue => write!(op, "{}", color::Bg(color::LightBlue)),
            EColor::LightMagenta => write!(op, "{}", color::Bg(color::LightMagenta)),
            EColor::LightCyan => write!(op, "{}", color::Bg(color::LightCyan)),
            EColor::LightWhite => write!(op, "{}", color::Bg(color::LightWhite)),
        }
    }
}