use super::{Skill, SkillId};
use crate::skills::combat_skills::broadsword::Broadsword;
use std::collections::HashMap;

pub struct SkillRegistry {
    skills: HashMap<SkillId, Box<dyn Skill>>
}

impl SkillRegistry {
    pub fn new() -> Self {
        let mut registry = SkillRegistry {
            skills: HashMap::new()
        };

        registry.register(Box::new(Broadsword));

        registry
    }

    fn register(&mut self, skill: Box<dyn Skill>) {
        self.skills.insert(skill.id(), skill);
    }

    pub fn get(&self, id: SkillId) -> Option<&dyn Skill> {
        self.skills.get(&id).map(|s| s.as_ref())
    }

}

impl Default for SkillRegistry {
    fn default() -> Self {
        Self::new()
    }
}
