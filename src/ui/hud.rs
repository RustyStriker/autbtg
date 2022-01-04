use termion::*;
use crate::{ivec2::IVec2};
use crate::entity::*;

use super::*;
use std::io;

pub struct Hud {
    pub game_name: String,
    pub bg_color: EColor,
    pub fg_color: EColor,
    pub turns: TurnBar,
    pub health: HealthBars,

}
impl Hud {
    const MAX_SIDE_LEN: u16 = 40;
    const MIN_SIDE_LEN: u16 = 15;

    pub fn draw<W: io::Write>(&self, op: &mut W, ents: &EntityMap) -> io::Result<()> {
        let (tx, ty) = terminal_size()?;
        let side_len = (tx / 3).min(Self::MAX_SIDE_LEN).max(Self::MIN_SIDE_LEN);
        let start = (tx - side_len + 2) as i32;
        let side_len = side_len - 2;
        // draw the box
        self.bg_color.write_bg(op)?;
        self.fg_color.write_fg(op)?;
        draw_box(op, 
            IVec2::new(start as i32 - 2, 1), 
            IVec2::new(tx as i32 + 1, ty as i32)
        )?;
        let mut cy = 2; // keep track of where we are in the Y axis
        // draw game name
        for l in self.game_name.lines() {
            if l.len() > side_len as usize {
                let (l1, l2) = l.split_at(side_len as usize - 2);
                write!(op, "{}{}", IVec2::new(start, cy).to_cursor(), l1)?;
                cy += 1;
                write!(op, "{}{}", IVec2::new(start, cy).to_cursor(), l2)?;
                cy += 1;
            }
            else {
                write!(op, "{}{}", IVec2::new(start, cy).to_cursor(), l)?;
                cy += 1;
            }
        }
        cy += 1; // space between stuff
        let health_iter = ents.iter()
            .map(|(id, e)| (id.symbol(), e.health(), e.max_health()));
        self.health.draw(op, IVec2::new(start, cy), side_len, health_iter)?;
        cy += ents.len() as i32 + 1; // yet again space between stuff
        


        Ok(())
    }
}
