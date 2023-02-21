// TODO: Idk why i made this maybe i will use it later

use bevy::prelude::*;

#[derive(Component)]
pub struct Acceleration {
    pub acc: Vec2,
}

impl Default for Acceleration {
    fn default() -> Self {
        Self { acc: Vec2::ZERO }
    }
}
