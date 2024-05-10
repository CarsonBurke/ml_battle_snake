pub mod graphics {
    pub const EMPTY: &str = "•";
    pub const FOOD: &str = "🍎";
    pub const HAZARD: &str = "⚠️";
    pub const HEAD: &str = "0";
    pub const BODY_UP: &str = "^";
    pub const BODY_DOWN: &str = "v";
    pub const BODY_LEFT: &str = "<";
    pub const BODY_RIGHT: &str = ">";
}

// The number of body parts the snake will have once it has fully moved on its original position
pub const SNAKE_STARTING_LENGTH: i32 = 3;