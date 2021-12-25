use std::io::Write;

use crate::game::Game;
use crate::ivec2::IVec2;
use crate::ui::boxx;
use termion::*;

const MIN_TERMINAL_SIZE: (u16,u16) = (92,22);

pub fn render<W: Write>(op: &mut W, _game: &Game) {
    let (termx, termy) = terminal_size().unwrap();

    if termx < MIN_TERMINAL_SIZE.0 || termy < MIN_TERMINAL_SIZE.1 {
        write!(op, "{}", termion::clear::All).unwrap();
        boxx::write_box(op, 
            &format!("{}x{} is too small of a terminal", termx,termy), 
            IVec2::new(5,5)
        ).unwrap();
    }
}