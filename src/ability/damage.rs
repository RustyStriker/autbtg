pub struct Damage {
    pub amount: i32,
    pub penetration: i32,
    pub magic: i32,
    pub damange_type: DamageType,
}


pub enum DamageType {
    Normal, Electric, Heat, Freeze, 
}