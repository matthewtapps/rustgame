use crate::core::attributes::AttributeType;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum SkillId {
    // Combat
    Broadsword,
    Shortsword,
    Rapier,
    Shield,
    Bow,
    Brawling,
    Karate,
    
    // Social
    FastTalk,
    Diplomacy,
    Intimidation,
    
    // Knowledge
    History,
    Theology,
    AreaKnowledge,
    
    // Athletic
    Climbing,
    Swimming,
    Stealth,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Difficulty {
    Easy,
    Average,
    Hard,
    VeryHard,
}

impl Difficulty {
    pub fn modifier(&self) -> i32 {
        match self {
            Difficulty::Easy => 0,
            Difficulty::Average => -1,
            Difficulty::Hard => -2,
            Difficulty::VeryHard => -3,
        }
    }
}

pub trait Skill {
    fn id(&self) -> SkillId;
    fn name(&self) -> &'static str;
    fn base_attribute(&self) -> AttributeType;
    fn difficulty(&self) -> Difficulty;
    fn defaults(&self) -> Vec<(SkillId, i32)>;

    fn points_for_level(&self, level_above_attribute: i32) -> i32 {
        match level_above_attribute {
            i if i <= 0 => 1,
            1 => 2,
            2 => 4,
            i => 4 + (i - 2) * 2,
        }
    }
}
