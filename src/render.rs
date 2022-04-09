use std::io::Write;

use crate::game::{Game, Chunk};
use crate::ivec2::IVec2;
use crate::ui::*;
use termion::*;

const MIN_TERMINAL_SIZE: (u16,u16) = (92,22);

pub fn render<W: Write>(op: &mut W, game: &Game, pos: IVec2) -> std::io::Result<()> {
    let (termx, termy) = terminal_size()?;

    if termx < MIN_TERMINAL_SIZE.0 || termy < MIN_TERMINAL_SIZE.1 {
        write!(op, "{}", termion::clear::All)?;
        write_box(op, 
            &format!("{}x{} is too small of a terminal", termx,termy), 
            IVec2::new(5,5)
        )?;
    }
    else {
        // Draw the world
        for (start, chunk) in game.world.chunks.iter() {
            render_chunk(op, chunk, *start + pos)?;
        }
        // Draw the hud
        game.hud.draw(op, &game.entities)?;
    }

    Ok(())
}

pub fn render_chunk<W: Write>(op: &mut W, chunk: &Chunk, start: IVec2) -> std::io::Result<()> {
    for y in 0..Chunk::SIZE as i32 {
        let mut continous = false;
        for x in 0..Chunk::SIZE as i32 {
            let (sx, sy) = terminal_size().unwrap_or((0,0));
            // make sure block is visible in screen
            let bigger_than_0 = start.x() + x > 0 && start.y() + y > 0;
            let less_than_screen = start.x() + x < sx as i32 && start.y() + y < sy as i32;
            if bigger_than_0 && less_than_screen {
                // we can render that block
                if !continous {
                    // we need to bump our cursor
                    write!(op, "{}", cursor::Goto((start.x() + x) as u16, (start.y() + y) as u16))?;
                    continous = true;
                }
                chunk.get(x as usize,y as usize).color().write_bg(op)?;
                write!(op, " ")?;
            }
            else {
                continous = false;
            }
        }
    }
    Ok(())
}