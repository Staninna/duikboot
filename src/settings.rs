use bevy::prelude::*;

// Player
pub mod player {
    use super::*;

    // Entity
    pub const NAME: &str = "Player";
    pub const GRAVITY_SCALE: f32 = 0.25;
    pub const TEXTURE: &str = "texture/submarine/submarine.png";
    pub const TEXTURE_SIZE: Vec2 = Vec2::new(32.0, 22.0);

    // Movement
    pub const MOVEMENT_SPEED_MULTIPLIER: f32 = 550.0;
    pub const FRICITON: f32 = 0.985;
    pub const MAX_SPEED: f32 = 200.0;
}

// Bubble
pub mod bubble {
    use super::*;

    // Entity
    pub const NAME: &str = "Bubble";
    pub const TEXTURE: &str = "texture/particles/bubble.png";
    pub const TEXTURE_SIZE: Vec2 = Vec2::new(8.0, 8.0);
}
