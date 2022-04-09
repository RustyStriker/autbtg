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
            text.push_str(&bb.to_str(self.bg_color, self.fg_color));

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
            text.push_str(&bb.to_str(self.bg_color, self.fg_color));

            write_box_center(op, &text, start)
        }
        else {
            write_box_center(op, text, start)
        }
    }

    /// Some(result) or none if invalid input
    pub fn on_input(&mut self, inp: Key) -> Option<bool> {
        if let Some(bb) = &mut self.buttons {
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
    pub yes_key: char,
    pub no_key: char,
    pub yes_selected: bool,
}
impl BoxButtons {
    pub const SPACE_BET_YES_NO: usize = 2;

    fn length(&self) -> usize {
        // 6 is for `(Y)` and `(n)` and such
        self.yes.len() + self.no.len() + Self::SPACE_BET_YES_NO + 6

    }

    fn to_str(&self, bg: EColor, fg: EColor) -> String {
        let mut s = String::with_capacity(self.length());
        if self.yes_selected {
            s.push_str(&format!("{}{}", fg.bg(), bg.fg()));
        }
        s.push_str(&self.yes);
        s.push('(');
        s.push(self.yes_key);
        s.push(')');
        s.push_str(&format!("{}{}", bg.bg(), fg.fg()));
        s.push_str(&" ".repeat(Self::SPACE_BET_YES_NO));
        if !self.yes_selected {
            s.push_str(&format!("{}{}", fg.bg(),bg.fg()));
        }
        s.push_str(&self.no);
        s.push('(');
        s.push(self.no_key);
        s.push(')');
        s.push_str(&format!("{}{}", fg.fg(),bg.bg()));

        s
    }

    fn on_input(&mut self, inp: Key) -> Option<bool> {
        if let Key::Char(c) = inp {
            if c == '\n' {
                // enter key, return default
                Some(self.yes_selected)
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
            if inp == Key::Left || inp == Key::Right {
                self.yes_selected = inp == Key::Left;
            }
            None
        }
    }
}


const PIPES: [char;6] = [ '╔', '═', '╗', '║', '╚', '╝'];
#[derive(Clone, Copy)]
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
impl std::fmt::Display for Pipes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

pub fn draw_box<W: Write>(op: &mut W, start: IVec2, end: IVec2) -> std::io::Result<()> {
    let size = end - start;

    // top part
    if start.y() >= 1 {
        write!(op, "{}{}{}{}", 
            cursor::Goto(start.x() as u16, start.y() as u16),
            Pipes::TopLeft,
            Pipes::Horizontal.to_string().repeat(size.x() as usize - 2),
            Pipes::TopRight
        )?;
    }
    // middle parts
    for i in 1..size.y() as u16 {
        write!(op, "{}{}{}{}", 
            cursor::Goto(start.x() as u16, start.y() as u16 + i),
            Pipes::Vertical,
            " ".repeat(size.x() as usize - 2),
            Pipes::Vertical
        )?;
    }
    if start.y() + size.y() <= terminal_size().map(|(_, y)| y).unwrap_or(0) as i32 {
        write!(op, "{}{}{}{}", 
            cursor::Goto(start.x() as u16, start.y() as u16 + size.y() as u16),
            Pipes::BottomLeft,
            Pipes::Horizontal.to_string().repeat(size.x() as usize - 2),
            Pipes::BottomRight
        )?;
    }
    op.flush()?;

    Ok(())
}

pub fn write_box<W: Write>(op: &mut W, message: &str, start: IVec2) -> std::io::Result<()> {
    // first break the string by new lines
    let lines = message.lines();
    let len =lines.clone()
        .map::<usize, _>(|l| strlen_without_esc(l))
        .max()
        .unwrap_or(0);
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
    let len = lines.clone()
        .map::<usize, _>(|l| strlen_without_esc(l))
        .max()
        .unwrap_or(0);
    let height = lines.clone().count();

    let end = start + IVec2::new(len as i32 + 4, height as i32 + 1);
    draw_box(op, start, end)?;

    for (i,l) in lines.enumerate() {
        let spaces = (len - strlen_without_esc(l)) / 2;
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

fn strlen_without_esc(str: &str) -> usize {
    str.len() - str.split("\x1b").map(|x| x.find('m').unwrap_or(0) + 2).sum::<usize>() + 2
}