use crate::skills::SkillId;
use crate::actors::character_sheet::CharacterSheet;

#[derive(Debug, Clone)]
pub struct Actor {
    pub id: String,
    pub name: String,
    pub sheet: CharacterSheet,
    pub position: (f32, f32),
    pub faction: Faction,
}

#[derive(Debug, Clone)]
pub enum Faction {
    Player,
    Ally,
    Neutral,
    Enemy
}

impl Actor {
    pub fn new(id: String, name: String) -> Self {
        Actor {
            id,
            name,
            sheet: CharacterSheet::new(),
            position: (0.0, 0.0),
            faction: Faction::Neutral,
        }
    }

    pub fn get_skill_level(&self, skill: SkillId) -> i32 {
        self.sheet.get_skill_level(skill)
    }

    pub fn recalculate_all(&mut self) {
        self.sheet.recalculate_all();
    }
}
