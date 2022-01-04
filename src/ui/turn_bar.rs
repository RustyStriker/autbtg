use std::io;
use termion::*;
use crate::ivec2::IVec2;
use super::{boxx::draw_box, EColor};

pub struct TurnBar {
    pub size: i32,
    pub bg_color: EColor,
    pub fg_color: EColor,

}
impl TurnBar {
    pub fn draw<'a, W, T>(
        &self,
        op: &mut W,
        times: T,
    ) -> io::Result<()>
    where
        W: io::Write,
        T: Iterator<Item = &'a (char,u32)> + Clone,
    {
        // apply the theme
        self.fg_color.write_fg(op)?;
        self.bg_color.write_bg(op)?;
        
        let term_size = terminal_size()
        .map(|(x,y)| IVec2::new(x as i32,y as i32))
        .unwrap_or(IVec2::ZERO);

        let skip = (term_size.x() - self.size - 4) / 2;
        let start = IVec2::new(skip,0);
        let end = IVec2::new(skip + self.size + 5, 2);
        // draw the surrounding box
        draw_box(op, start, end)?;
        // draw the bar in the middle
        let bar_start = IVec2::new(skip + 2, 1);
        let bar_len = self.size - 1;
        // draw the bar itself
        write!(op, "{}#{}", 
            bar_start.to_cursor(),
            "-".repeat(bar_len as usize),
        )?;
        let max = times.clone().map(|t| t.1).max().unwrap_or(0);

        // write the times themselves
        for (c, t) in times {
            let pos = *t as f32 / max as f32;
            let pos = pos * bar_len as f32;
            
            write!(op, "{}{}",
            (bar_start + IVec2::new(pos as i32 + 1, 0)).to_cursor(),
            c
        )?;
        }

        op.flush()?;


        Ok(())
    }
}