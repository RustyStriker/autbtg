use std::io::Write;
use termion::*;

use crate::ivec2::IVec2;


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