use crate::skills::{Skill, SkillId, Difficulty};
use crate::core::attributes::AttributeType;

pub struct Shortsword;

impl Skill for Shortsword {
    fn id(&self) -> SkillId { SkillId::Shortsword }
    fn name(&self) -> &'static str { "Shortsword" }
    fn base_attribute(&self) -> AttributeType { AttributeType::Dexterity }
    fn difficulty(&self) -> Difficulty { Difficulty::Average }

    fn defaults(&self) -> Vec<(SkillId, i32)> {
        vec![
            (SkillId::Broadsword, -2),
            (SkillId::Rapier, -4),
        ]
    }
}
