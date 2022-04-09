use crate::entity::Entity;

pub trait Effect {
    fn apply(&mut self, entity: &dyn Entity);
}