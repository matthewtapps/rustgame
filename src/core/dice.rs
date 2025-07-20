use rand::Rng;

pub struct DiceRoller; 

impl DiceRoller {
    pub fn roll_3d6() -> i32 {
        let mut rng = rand::rng();
        (1..=3).map(|_| rng.random_range(1..=6)).sum()
    }

    pub fn success_check(target: i32) -> (bool, i32, i32) {
        let roll = Self::roll_3d6();
        let success = roll <= target;
        let margin = target - roll;
        (success, roll, margin)
    }
}
