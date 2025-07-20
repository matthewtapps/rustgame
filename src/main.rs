use rustgame::combat::combat_manager::CombatManager;
use rustgame::actors::actor::Faction;
use rustgame::skills::SkillId;
use rustgame::actors::actor::Actor;



fn main() {
let mut player = Actor::new("player_1".to_string(), "Sir Roland".to_string());
    
    // Improve attributes
    player.sheet.attributes.dexterity = 13;
    player.sheet.attributes.strength = 12;
    player.sheet.recalculate_all(); // Update derived values
    
    // Learn and improve skills
    player.sheet.improve_skill(SkillId::Broadsword, 8);
    player.sheet.improve_skill(SkillId::Shield, 4);
    
    // Check current skill level (cached)
    let sword_level = player.get_skill_level(SkillId::Broadsword);
    println!("{} has Broadsword at level {}", player.name, sword_level);
    
    // Combat example
    let mut goblin = Actor::new("goblin_1".to_string(), "Goblin Warrior".to_string());
    goblin.faction = Faction::Enemy;
    
    let combat_mgr = CombatManager {
        actors: vec![player.clone(), goblin.clone()],
        current_time: 0.0,
    };
    
    let result = combat_mgr.resolve_attack(&player, &goblin, SkillId::Broadsword);
    println!("Attack result: {:?}", result);
}
