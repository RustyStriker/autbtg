use super::*;

pub struct ModifiedAbility<'a> {
    pub base: &'a dyn Ability,
    pub size_mult: f32,
    pub damage: damage::Damage,
}
impl Ability for ModifiedAbility<'_> {
    fn size(&self) -> AbilitySize {
        self.base.size().mult_size(self.size_mult)
    }

    fn damage(&self) -> damage::Damage {
        let dmg = self.base.damage();
        damage::Damage { 
            amount: dmg.amount,
            ..dmg
        }
    }
}