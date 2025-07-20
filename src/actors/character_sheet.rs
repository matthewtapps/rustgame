use std::collections::HashMap;

use crate::{core::{attributes::Attributes, modifiers::Modifier}, skills::SkillId};

#[derive(Debug, Clone)]
pub struct CharacterSheet {
    pub attributes: Attributes,
    pub skill_points: HashMap<SkillId, i32>,
    pub max_hp: i32,
    pub max_fatigue: i32,
    pub basic_speed: f32,
    pub movement: i32,
    pub dodge: i32,
    pub current_hp: i32,
    pub current_fatigue: i32,
    skill_levels: HashMap<SkillId, i32>,
    pub modifiers: Vec<Modifier>,
}

impl CharacterSheet {
    pub fn new() -> Self {
        let attributes = Attributes::new();
        let mut sheet = CharacterSheet {
            attributes,
            skill_points: HashMap::new(),
            max_hp: 0,
            max_fatigue: 0,
            basic_speed: 0.0,
            movement: 0,
            dodge: 0,
            current_hp: 0,
            current_fatigue: 0,
            skill_levels: HashMap::new(),
            modifiers: Vec::new(),
        };

        sheet.recalculate_all();
        sheet
    }

    pub fn recalculate_all(&mut self) {
        self.recalculate_secondary_characteristics();
        self.recalculate_all_skills();
    }

    fn recalculate_secondary_characteristics(&mut self) {
        self.max_hp = self.attributes.strength;
        self.max_fatigue = self.attributes.health;
        self.basic_speed = (self.attributes.dexterity + self.attributes.health) as f32 / 4.0;
        self.movement = self.basic_speed.floor() as i32;
        self.dodge = self.movement + 3;

        self.current_hp = self.current_hp.min(self.max_hp);
        self.current_fatigue = self.current_fatigue.min(self.max_fatigue);
    }

    fn recalculate_all_skills(&mut self) {
        // This is where we'd use the SkillRegistry to calculate each skill
        // For now, simplified calculation
        for (skill_id, points) in &self.skill_points {
            let level = self.calculate_skill_level(&skill_id, *points);
            self.skill_levels.insert(skill_id.clone(), level);
        }
    }
    
    fn calculate_skill_level(&self, skill_id: &SkillId, points: i32) -> i32 {
        // In a real implementation, this would:
        // 1. Look up the skill definition
        // 2. Get the base attribute
        // 3. Apply difficulty modifier
        // 4. Check for defaults from other skills
        // 5. Return the best available level
        
        // Simplified for now
        if points == 0 {
            return self.get_default_level(skill_id.clone());
        }
        
        // Basic GURPS progression (simplified)
        let base_attr = 10; // Would look this up
        let difficulty_mod = -1; // Would get from skill
        
        let bonus = match points {
            1 => 0,
            2 => 1,
            4 => 2,
            p if p >= 8 => 2 + (p - 4) / 4,
            _ => 1,
        };
        
        base_attr + difficulty_mod + bonus
    }
    
    fn get_default_level(&self, skill_id: SkillId) -> i32 {
        // Check if any known skills provide a default
        // Simplified - would check skill registry for defaults
        0
    }
    
    pub fn get_skill_level(&self, skill: SkillId) -> i32 {
        self.skill_levels.get(&skill).copied().unwrap_or(0)
    }
    
    pub fn improve_skill(&mut self, skill: SkillId, points: i32) {
        let current = self.skill_points.get(&skill).copied().unwrap_or(0);
        self.skill_points.insert(skill.clone(), current + points);
        
        // Recalculate just this skill and any that default from it
        self.recalculate_skill(skill);
    }
    
    fn recalculate_skill(&mut self, skill: SkillId) {
        let points = self.skill_points.get(&skill).copied().unwrap_or(0);
        let level = self.calculate_skill_level(&skill, points);
        self.skill_levels.insert(skill, level);
        
        // Also recalculate any skills that might default from this one
        // In a real implementation, we'd track these relationships
    }
}
