use crate::prelude::*;

#[derive(Resource)]
pub struct CharsetAsset {
    pub atlas: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct MovementState {
    pub is_moving: bool,
}

#[derive(Resource, Default)]
pub struct GameTick {
    pub current: u64,
}

impl GameTick {
    pub fn advance(&mut self) {
        self.current += 1;
    }
}
#[derive(Resource)]
pub struct PlayerTurn {
    pub action_points: i32,
    pub max_action_points: i32,
    pub turn_active: bool,
}

impl Default for PlayerTurn {
    fn default() -> Self {
        Self {
            action_points: 10, // Starting AP, adjust based on your design
            max_action_points: 10,
            turn_active: true,
        }
    }
}

impl PlayerTurn {
    pub fn can_afford(&self, cost: i32) -> bool {
        self.action_points >= cost && self.turn_active
    }

    pub fn spend(&mut self, cost: i32) -> bool {
        if self.can_afford(cost) {
            self.action_points -= cost;
            true
        } else {
            false
        }
    }

    pub fn end_turn(&mut self) {
        self.turn_active = false;
    }

    pub fn start_new_turn(&mut self) {
        self.action_points = self.max_action_points;
        self.turn_active = true;
    }
}
