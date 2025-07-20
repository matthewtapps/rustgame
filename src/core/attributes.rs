#[derive(Debug, Clone)]
pub struct Attributes {
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub health: i32
}

pub enum AttributeType {
    Strength,
    Dexterity,
    Intelligence,
    Health
}

impl Attributes {
    pub fn new() -> Self {
        Attributes { strength: 10, dexterity: 10, intelligence: 10, health: 10 }
    }

    pub fn get_attribute(&self, attr: AttributeType) -> i32 {
        match attr {
            AttributeType::Health => self.health,
            AttributeType::Strength => self.strength,
            AttributeType::Dexterity => self.dexterity,
            AttributeType::Intelligence => self.intelligence
        }
    }
}

impl Default for Attributes {
    fn default() -> Self { Self::new() }
}
