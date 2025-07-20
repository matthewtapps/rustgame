use crate::actors::actor::Actor;
use crate::skills::SkillId;
use crate::core::dice::DiceRoller;

pub struct CombatManager {
    pub actors: Vec<Actor>,
    pub current_time: f32,
}

impl CombatManager {
    pub fn resolve_attack(
        &self,
        attacker: &Actor,
        defender: &Actor,
        weapon_skill: SkillId,
    ) -> AttackResult {
        let skill_level = attacker.get_skill_level(weapon_skill);
        
        // Apply modifiers (range, lighting, etc.)
        let effective_skill = skill_level; // Would apply modifiers here
        
        let (success, roll, margin) = DiceRoller::success_check(effective_skill);
        
        if !success {
            return AttackResult::Miss { roll, target: effective_skill };
        }
        
        // Check defense
        let dodge = defender.sheet.dodge;
        let (defended, def_roll, _) = DiceRoller::success_check(dodge);
        
        if defended {
            AttackResult::Defended { 
                attack_roll: roll,
                defense_roll: def_roll,
                defense_type: "Dodge".to_string(),
            }
        } else {
            // Calculate damage based on weapon and margin
            let damage = 5 + margin.max(0) / 3; // Simplified
            AttackResult::Hit { 
                damage,
                location: "Torso".to_string(), // Would randomize
            }
        }
    }
}

#[derive(Debug)]
pub enum AttackResult {
    Miss { roll: i32, target: i32 },
    Defended { attack_roll: i32, defense_roll: i32, defense_type: String },
    Hit { damage: i32, location: String },
}
