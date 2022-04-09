use std::io::{self, Write};
use std::thread;
use autbtg::*;

use ivec2::IVec2;
use ui::EColor;
use termion::*;
use termion::raw::IntoRawMode;
use termion::event::{Key, Event};
use termion::input::{TermRead, MouseTerminal};


fn main() {

    let v = IVec2::new(10,21);
    println!("{}",v);

    let stdin = termion::async_stdin();
    let stdout = io::stdout();
    let mut stdout = MouseTerminal::from(stdout).into_raw_mode().unwrap();
    
    write!(stdout, "{}", cursor::Hide).unwrap();

    // Wait until the screen is big enough(i wanna say 20x10 is enough but aint sure)
    write!(stdout, "{}", clear::All).unwrap();

    let mut chunk = game::Chunk::new(game::Block::Dirt);
    (4..=15).for_each(|x| {
        (4..=15).for_each(|y| {
            if x == 15 || x == 4 || y == 15 || y == 4 || x == y {
                chunk.set(game::Block::Wall, x, y);
            }
        });
    });
    
    let mut popbox = ui::PopBox {
        bg_color: EColor::Black,
        fg_color: EColor::White,
        buttons: Some(ui::BoxButtons {
            yes: "yes".to_string(),
            no: "no".to_string(),
            yes_selected: true,
            yes_key: 'y',
            no_key: 'n',
        }),
    };

    let tb = ui::TurnBar {
        size: 20,
        bg_color: EColor::Black,
        fg_color: EColor::Cyan,
    };
    let hb = ui::HealthBars {
        bg: EColor::Black,
        name: EColor::White,
        health_low: EColor::Red,
        health_mid: EColor::Yellow,
        health_high: EColor::Green,
    };
    let hud = ui::Hud {
        game_name: "This is a very cool game name\n cant you see how cool it is???".to_string(),
        bg_color: EColor::Black,
        fg_color: EColor::White,
        turns: tb,
        health: hb,
    };
    let mut ents = entity::EntityMap::new();
    ents.insert(
        entity::EntityId::new('@'), 
        Box::new(entity::Spider::new(IVec2::ZERO))
    );
    ents.get_mut(&entity::EntityId::new('@')).unwrap().add_health(-2);
    
    let mut add = false;

    // Actual loop test with "rEaL tImE rEnDeRiNg"
    let mut events = stdin.events();
    loop {
        write!(stdout, "{}", clear::All).unwrap();
        if let Some(e) = events.next() {
            if let Ok(e) = e {
                if let Event::Key(Key::Esc) = e {
                    break;
                }
                else if let Event::Key(k) = e {
                    if popbox.on_input(k).unwrap_or(false) {
                        break;
                    }
                }
            }
        }
        let add_am = if add { 1 } else { -1 };
        add = !add;
        ents.get_mut(&entity::EntityId::new('@')).unwrap().add_health(add_am);
        render::render_chunk(&mut stdout, &chunk, IVec2::new(1,1)).unwrap();
        hud.draw(&mut stdout, &ents).unwrap();
        popbox.draw_center(&mut stdout, IVec2::new(16,16), "Would you like to quitt?").unwrap();
        stdout.flush().unwrap();
        thread::sleep(std::time::Duration::from_millis(25));
    }

    println!("{}{}{}Goodbye :)", 
        cursor::Goto(1, terminal_size().map(|(_,y)| y - 4).unwrap_or(1)),
        color::Bg(color::Black),
        color::Fg(color::White)
    );
}
