use bevy::prelude::*;

// Player
pub mod player {
    use super::*;

    // Texture
    pub const TEXTURE: &str = "texture/submarine/submarine.png";
    pub const TEXTURE_SIZE: Vec2 = Vec2::new(32.0, 22.0);
    pub const TEXTURE_SHEET_SIZE: [usize; 2] = [1, 1];
    pub const TEXTURE_PADDING: Option<Vec2> = None;
    pub const TEXTURE_OFFSET: Option<Vec2> = None;

    // Entity
    pub const NAME: &str = "Player";
    pub const GRAVITY_SCALE: f32 = 0.25;

    // Movement
    pub const MOVEMENT_SPEED_MULTIPLIER: f32 = 550.0;
    pub const FRICITON: f32 = 0.985;
    pub const MAX_SPEED: f32 = 200.0;
    pub const MIN_SPEED: f32 = 0.0;

    // Trail
    pub const TRAIL_NAME: &str = "Bubble Trail";
    pub const TRAIL_TICK: f32 = 0.1;
    pub const TRAIL_LIFETIME: f32 = 1.5;
    pub const TRAIL_VELOCITY_MULTIPLIER: f32 = -0.15;
    pub const TRAIL_RANDOM_VELOCITY_MAX: f32 = 10.0;
    pub const TRAIL_RANDOM_VELOCITY_MIN: f32 = -10.0;

    // Keys
    pub const KEY_UP: KeyCode = KeyCode::W;
    pub const KEY_DOWN: KeyCode = KeyCode::S;
    pub const KEY_LEFT: KeyCode = KeyCode::A;
    pub const KEY_RIGHT: KeyCode = KeyCode::D;
}

// Bubble
pub mod bubble {
    use super::*;

    // Entity
    pub const TEXTURE: &str = "texture/particles/bubble.png";
    pub const TEXTURE_SIZE: Vec2 = Vec2::new(8.0, 8.0);
}
