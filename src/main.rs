use std::io::{self, Write};

use gameski::ivec2::IVec2;
use gameski::ui::EColor;
use termion::*;
use termion::raw::IntoRawMode;
use termion::event::{Key, Event};
use termion::input::{TermRead, MouseTerminal};


fn main() {

    let v = IVec2::new(10,21);
    println!("{}",v);

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = MouseTerminal::from(stdout).into_raw_mode().unwrap();
    
    write!(stdout, "{}", cursor::Hide).unwrap();

    // Wait until the screen is big enough(i wanna say 20x10 is enough but aint sure)
    write!(stdout, "{}", clear::All).unwrap();

    gameski::ui::boxx::write_box(&mut stdout,
        "This message is normal\nsee?", 
        IVec2::new(20,5)
    ).unwrap();
    gameski::ui::boxx::write_box_center(&mut stdout,
        "This message should center iteself\nhopefully\nomg omg omg omg omg omg omg omg omg omg\nit worksssss",
        IVec2::new(20,11)
    ).unwrap();

    gameski::ui::turn_bar::draw_turn_bar(&mut stdout, 
        [('A', 2), ('@', 5), ('B', 8), ('C', 4)].iter(), 
        gameski::ui::turn_bar::TurnBar {
            size: 20,
            bg_color: EColor::Black,
            fg_color: EColor::Cyan,
        },
    ).unwrap();

    for e in stdin.events() {
        if let Ok(e) = e {
            if let Event::Key(Key::Esc) = e {
                break;
            }
        }
    }

    println!("{}Goodbye :)", cursor::Goto(1,20));
}
