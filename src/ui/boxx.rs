use std::io::Write;
use termion::*;
use termion::event::Key;

use crate::ivec2::IVec2;

use super::EColor;

pub struct PopBox {
    pub bg_color: EColor,
    pub fg_color: EColor,
    pub buttons: Option<BoxButtons>,
}
impl PopBox {
    pub fn draw<W: Write>(&self, op: &mut W, start: IVec2, text: &str,) -> std::io::Result<()> {
        // apply theme
        self.bg_color.write_bg(op)?;
        self.fg_color.write_fg(op)?;

        if let Some(bb) = &self.buttons {
            let mut text = text.to_string();
            text.push('\n');
            text.push_str(&bb.to_str());

            write_box(op, &text, start)
        }
        else {
            write_box(op, text, start)
        }
    }
    pub fn draw_center<W: Write>(&self, op: &mut W, start: IVec2, text: &str,) -> std::io::Result<()> {
        // apply theme
        self.bg_color.write_bg(op)?;
        self.fg_color.write_fg(op)?;

        if let Some(bb) = &self.buttons {
            let mut text = text.to_string();
            text.push('\n');
            text.push_str(&bb.to_str());

            write_box_center(op, &text, start)
        }
        else {
            write_box_center(op, text, start)
        }
    }

    /// Some(result) or none if invalid input
    pub fn on_input(&self, inp: Key) -> Option<bool> {
        if let Some(bb) = &self.buttons {
            bb.on_input(inp)
        }
        else {
            Some(false)
        }
    }
}
pub struct BoxButtons {
    pub yes: String,
    pub no: String,
    pub default_yes: bool,
    pub yes_key: char,
    pub no_key: char,

}
impl BoxButtons {
    pub const SPACE_BET_YES_NO: usize = 2;

    fn length(&self) -> usize {
        // 6 is for `(Y)` and `(n)` and such
        self.yes.len() + self.no.len() + Self::SPACE_BET_YES_NO + 6

    }

    fn to_str(&self) -> String {
        let mut s = String::with_capacity(self.length());
        s.push_str(&self.yes);
        s.push('(');
        if self.default_yes {
            s.push(self.yes_key.to_ascii_uppercase());
        }
        else {
            s.push(self.yes_key.to_ascii_lowercase());
        }
        s.push(')');
        s.push_str(&" ".repeat(Self::SPACE_BET_YES_NO));
        s.push_str(&self.no);
        s.push('(');
        if self.default_yes {
            s.push(self.no_key.to_ascii_lowercase());
        }
        else {
            s.push(self.no_key.to_ascii_uppercase());
        }
        s.push(')');

        s
    }

    fn on_input(&self, inp: Key) -> Option<bool> {
        if let Key::Char(c) = inp {
            if c == '\n' {
                // enter key, return default
                Some(self.default_yes)
            }
            else if c == self.yes_key {
                Some(true)
            }
            else if c == self.no_key {
                Some(false)
            }
            else {
                None
            }
        }
        else {
            None
        }
    }
}


const PIPES: [char;6] = [ '╔', '═', '╗', '║', '╚', '╝'];
enum Pipes {
    TopLeft,
    Horizontal,
    TopRight,
    Vertical,
    BottomLeft,
    BottomRight,
}
impl Pipes {
    fn get(self) -> char {
        match self {
            Pipes::TopLeft => PIPES[0],
            Pipes::Horizontal => PIPES[1],
            Pipes::TopRight => PIPES[2],
            Pipes::Vertical => PIPES[3],
            Pipes::BottomLeft => PIPES[4],
            Pipes::BottomRight => PIPES[5],
        }
    }
}

pub fn draw_box<W: Write>(op: &mut W, start: IVec2, end: IVec2) -> std::io::Result<()> {
    let size = end - start;

    // top part
    if start.y() >= 1 {
        write!(op, "{}{}{}{}", 
            cursor::Goto(start.x() as u16, start.y() as u16),
            Pipes::TopLeft.get(),
            Pipes::Horizontal.get().to_string().repeat(size.x() as usize - 2),
            Pipes::TopRight.get()
        )?;
    }
    // middle parts
    for i in 1..size.y() as u16 {
        write!(op, "{}{}{}{}", 
            cursor::Goto(start.x() as u16, start.y() as u16 + i),
            Pipes::Vertical.get(),
            " ".repeat(size.x() as usize - 2),
            Pipes::Vertical.get()
        )?;
    }
    if start.y() + size.y() <= terminal_size().map(|(_, y)| y).unwrap_or(0) as i32 {
        write!(op, "{}{}{}{}", 
            cursor::Goto(start.x() as u16, start.y() as u16 + size.y() as u16),
            Pipes::BottomLeft.get(),
            Pipes::Horizontal.get().to_string().repeat(size.x() as usize - 2),
            Pipes::BottomRight.get()
        )?;
    }
    op.flush()?;

    Ok(())
}

pub fn write_box<W: Write>(op: &mut W, message: &str, start: IVec2) -> std::io::Result<()> {
    // first break the string by new lines
    let lines = message.lines();
    let len = lines.clone().max_by_key(|l| l.len()).map(|l| l.len()).unwrap_or(0);
    let height = lines.clone().count();

    // draw the box
    let end = start + IVec2::new(len as i32 + 4, height as i32 + 1);
    draw_box(op, start, end)?;

    // write the actual text
    for (i, l) in lines.enumerate() {
        write!(op, "{} {} ",
            cursor::Goto(start.x() as u16 + 1, start.y() as u16 + i as u16 + 1),
            l
        )?;
    }
    op.flush()?;
    Ok(())
}
/// Writes a text box on the screen with the text centered in the box
pub fn write_box_center<W: Write>(op: &mut W, message: &str, start: IVec2) -> std::io::Result<()> {
    let lines = message.lines();
    let len = lines.clone().max_by_key(|l| l.len()).map(|l| l.len()).unwrap_or(0);
    let height = lines.clone().count();

    let end = start + IVec2::new(len as i32 + 4, height as i32 + 1);
    draw_box(op, start, end)?;

    for (i,l) in lines.enumerate() {
        let spaces = (len - l.len()) / 2;
        write!(op, "{} {}{}{} ", 
            cursor::Goto(start.x() as u16 + 1, start.y() as u16 + i as u16 + 1),
            " ".repeat(spaces),
            l,
            " ".repeat(spaces),
        )?;
    }
    op.flush()?;
    Ok(())
}