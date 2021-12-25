mod spider;
pub use spider::*;

use crate::ivec2::IVec2;


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId {
    symbol: char,
}
impl EntityId {
    pub fn new(symbol: char) -> Self {
        Self { symbol }
    }
    pub fn symbol(&self) -> char {
        self.symbol
    }
}

/// # Entity
/// Entities are defined as living stuff
/// 
/// An Entity should have the ability to:
/// - Play turns
/// - Have a turn timer
/// - Health
/// - Position and Movement
/// - Visibility
/// - Faction(Players/Enemies/Something else)
/// - maybe more?
pub trait Entity {
    // turns
    fn play(&mut self, entities: Vec<&mut Box<dyn Entity>>,);
    fn timer(&self) -> u32;
    fn advance_timer(&mut self, amount: u32);
    // health
    fn health(&self) -> i32;
    fn add_health(&mut self, amount: i32);
    // position
    fn position(&self) -> IVec2;
    fn set_position(&mut self, new: IVec2);
    fn move_by(&mut self, amount: IVec2);
    // visibility
    fn visible(&self) -> bool;
    // faction
    fn faction(&self) -> Faction;
    
}

#[derive(Clone, Copy)]
pub enum Faction {
    Players, Spiders
}