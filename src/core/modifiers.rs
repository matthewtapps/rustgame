#[derive(Debug, Clone)]
pub struct Modifier {
    pub source: String,
    pub value: i32,
    pub duration: ModifierDuration,
}

#[derive(Debug, Clone)]
pub enum ModifierDuration {
    Permanent,
    Temporary { remaining_seconds: f32 },
    Conditional { condition: String }
}
