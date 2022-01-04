use crate::ivec2::IVec2;

use super::Entity;



pub struct Spider {
    timer: u32,
    health: i32,
    pos: IVec2,
}
impl Spider {
    pub const MAX_HEALTH: i32 = 10;

    pub fn new(pos: IVec2) -> Self {
        Self {
            health: Self::MAX_HEALTH,
            pos,
            timer: 5,
        }
    }
}
impl Entity for Spider {
    fn play(&mut self, _entities: Vec<&mut Box<dyn Entity>>,) {
        todo!()
    }

    fn timer(&self) -> u32 {
        self.timer
    }

    fn advance_timer(&mut self, amount: u32) {
        self.timer -= amount
    }

    fn health(&self) -> i32 {
        self.health
    }

    fn max_health(&self) -> i32 {
        Self::MAX_HEALTH
    }

    fn add_health(&mut self, amount: i32) {
        self.health += amount;
    }

    fn position(&self) -> crate::ivec2::IVec2 {
        self.pos
    }

    fn set_position(&mut self, new: crate::ivec2::IVec2) {
        self.pos = new;
    }

    fn move_by(&mut self, amount: crate::ivec2::IVec2) {
        self.pos = self.pos + amount;
    }

    fn visible(&self) -> bool {
        false
    }

    fn faction(&self) -> super::Faction {
        super::Faction::Spiders
    }
}