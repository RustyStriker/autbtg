use crate::ivec2::IVec2;

use super::*;
use std::io;

pub struct HealthBars {
    pub bg: EColor,
    pub name: EColor,
    pub health_low: EColor,
    pub health_mid: EColor,
    pub health_high: EColor,
}
impl HealthBars {
    pub fn draw<W, T>(
        &self,
        op: &mut W,
        from: IVec2, 
        len: u16, 
        healths: T
    ) -> io::Result<()>
    where
        W: io::Write,
        T: Iterator<Item = (char, i32, i32)> + Clone,
    {
        // Example:
        // @[#######      ]
        // A[#########    ]
        // B[#####        ]
        assert!(len > 3, "HealthBars len must be bigger than 3!");
        // apply background color
        self.bg.write_bg(op)?;
        let bar = len - 8;
        let low = bar / 3;
        let mid = 2 * bar / 3;

        for (i, (c, h, mh)) in healths.enumerate() {
            // the amount of # in the health bar
            let hl = h as u16 * bar / mh as u16;
            // write the symbol
            self.name.write_fg(op)?;
            write!(op, "{}{}[", (from + IVec2::new(0, i as i32)).to_cursor(), c)?;
            // set the color for the health
            if hl < low {
                self.health_low.write_fg(op)?;
            }
            else if hl < mid {
                self.health_mid.write_fg(op)?;
            }
            else {
                self.health_high.write_fg(op)?;
            }
            write!(op, "{}{}", "#".repeat(hl as usize), " ".repeat((bar - hl) as usize))?;
            self.name.write_fg(op)?;
            write!(op, "]{: >2}/{}", h, mh)?;

        }
        Ok(())
    }
}
