use crate::ivec2::IVec2;

use super::Entity;



pub struct Spider {
    timer: u32,
    health: i32,
    position: IVec2,
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

    fn add_health(&mut self, amount: i32) {
        self.health += amount;
    }

    fn position(&self) -> crate::ivec2::IVec2 {
        self.position
    }

    fn set_position(&mut self, new: crate::ivec2::IVec2) {
        self.position = new;
    }

    fn move_by(&mut self, amount: crate::ivec2::IVec2) {
        self.position = self.position + amount;
    }

    fn visible(&self) -> bool {
        false
    }

    fn faction(&self) -> super::Faction {
        super::Faction::Spiders
    }
}