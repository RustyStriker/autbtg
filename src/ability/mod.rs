pub mod damage;
pub mod effect;
mod modified_ability;
pub use modified_ability::*;
pub trait Ability {
    // a lot of "info" based stuff - maybe put it in 1 function and get a struct from it?
    fn size(&self) -> AbilitySize;
    fn damage(&self) -> damage::Damage;
}
pub trait AbilityModifier {
    /// Applies the modifier on an ability
    fn apply<'a>(&self, ability: &'a dyn Ability) -> ModifiedAbility<'a>;
}




#[derive(Clone, Copy, Debug,)]
pub enum AbilitySize {
    AOECircle {
        radius: u16,
    },
    AOECone {
        length: u16,
        width: u16,
    },
    AOESquare {
        length: u16,
        width: u16,
    },
    Shot {
        range: u16,
    },
}
impl AbilitySize {
    pub fn mult_size(self, mult: f32) -> AbilitySize {
        match self {
            AbilitySize::AOECircle { radius } => 
                AbilitySize::AOECircle { radius: (radius as f32 * mult) as u16 },
            AbilitySize::AOECone { length, width } => 
                AbilitySize::AOECone { length: (length as f32 * mult) as u16, width },
            AbilitySize::AOESquare { length, width } => 
                AbilitySize::AOESquare { 
                    length: (length as f32 * mult) as u16, 
                    width: (width as f32 * mult) as u16
                },
            AbilitySize::Shot { range } => 
                AbilitySize::Shot { range: (range as f32 * mult) as u16 },
        }
    }
}
/*
Abilities:
    - Size
    - Modifiers? Will be saved differently and abilities pass through them like a pipe
    - Cooldown and stuff
    - Cooldown should be easy and straight forward
    
*/