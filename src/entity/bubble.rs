use crate::{
    resource::texture::TextureAtlasResource,
    settings::bubble::{
        TEXTURE, TEXTURE_OFFSET, TEXTURE_PADDING, TEXTURE_SHEET_SIZE, TEXTURE_SIZE,
    },
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BubblePlugin;
impl Plugin for BubblePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets)
            .add_system(movement)
            .add_system(lifetime);
    }
}

#[derive(Clone, Debug)]
pub enum BubbleType {
    Normal,
    Big,
    Small,
}

impl BubbleType {
    pub fn random() -> Self {
        let random = rand::random::<u8>() % 3;

        match random {
            0 => Self::Normal,
            1 => Self::Big,
            2 => Self::Small,
            _ => unreachable!("Random number is 0, 1, or 2"),
        }
    }
}

#[derive(Component, Debug)]
pub struct Bubble {
    lifetime: f32,
    time_left: f32,
}

impl Bubble {
    pub fn new(lifetime: f32) -> Self {
        Self {
            lifetime,
            time_left: lifetime,
        }
    }
}

fn load_assets(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    // Load/save texture
    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load(TEXTURE),
        TEXTURE_SIZE,
        TEXTURE_SHEET_SIZE[0],
        TEXTURE_SHEET_SIZE[1],
        TEXTURE_PADDING,
        TEXTURE_OFFSET,
    );
    let handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(TextureAtlasResource::new(handle));
}

fn movement(time: Res<Time>, mut query: Query<(&Bubble, &mut Transform, &Velocity)>) {
    for (_, mut transform, velocity) in query.iter_mut() {
        // move bubble based on lifetime, velocity, and time
        let delta = time.delta_seconds();
        let x = transform.translation.x + velocity.linvel.x * delta;
        let y = transform.translation.y + velocity.linvel.y * delta;
        transform.translation = Vec3::new(x, y, 0.0);
    }
}

fn lifetime(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Bubble, &mut TextureAtlasSprite)>,
) {
    for (entity, mut bubble, mut sprite) in query.iter_mut() {
        // decrease lifetime
        bubble.time_left -= time.delta_seconds();

        // remove bubble if lifetime is 0
        if bubble.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        } else {
            let some_color = &mut sprite.color;
            some_color.set_a(bubble.time_left / bubble.lifetime);
        }
    }
}
