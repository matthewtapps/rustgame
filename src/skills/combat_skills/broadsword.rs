use crate::skills::{Skill, SkillId, Difficulty};
use crate::core::attributes::AttributeType;

pub struct Broadsword;

impl Skill for Broadsword {
    fn id(&self) -> SkillId { SkillId::Broadsword }
    fn name(&self) -> &'static str { "Broadsword" }
    fn base_attribute(&self) -> AttributeType { AttributeType::Dexterity }
    fn difficulty(&self) -> Difficulty { Difficulty::Average }

    fn defaults(&self) -> Vec<(SkillId, i32)> {
        vec![
            (SkillId::Shortsword, -2),
            (SkillId::Rapier, -4),
        ]
    }
}
