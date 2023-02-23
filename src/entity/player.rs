//TODO: Add comments - https://trello.com/c/8HXAWr31/9-add-comments

use crate::{
    components::acceleration::Acceleration,
    entity::bubble::Bubble,
    resource::texture::TextureAtlasResource,
    settings::player::{
        FRICITON, GRAVITY_SCALE, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP, MAX_SPEED, MIN_SPEED,
        MOVEMENT_SPEED_MULTIPLIER, NAME, TEXTURE, TEXTURE_OFFSET, TEXTURE_PADDING,
        TEXTURE_SHEET_SIZE, TEXTURE_SIZE, TRAIL_LIFETIME, TRAIL_NAME, TRAIL_RANDOM_VELOCITY_MAX,
        TRAIL_RANDOM_VELOCITY_MIN, TRAIL_TICK, TRAIL_VELOCITY_MULTIPLIER,
    },
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use super::bubble::BubbleType;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(movement)
            .add_system(rotation)
            .add_system(trail);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Trail {
    timer: Timer,
}

impl Trail {
    pub fn new(lifetime: f32) -> Self {
        Trail {
            timer: Timer::from_seconds(lifetime, TimerMode::Repeating),
        }
    }
}

//TODO: Clean up spawn_player - https://trello.com/c/wBqebvSr/6-clean-up-spawnplayer
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    // Create components

    // Defined in settings.rs
    let name = Name::new(NAME);
    let gravity = GravityScale(GRAVITY_SCALE);
    let trail_timer = Trail::new(TRAIL_TICK);
    let atlas = TextureAtlas::from_grid(
        asset_server.load(TEXTURE),
        TEXTURE_SIZE,
        TEXTURE_SHEET_SIZE[0],
        TEXTURE_SHEET_SIZE[1],
        TEXTURE_PADDING,
        TEXTURE_OFFSET,
    );

    // Defined in this file
    let body_type = RigidBody::Dynamic;
    let velocity = Velocity::zero();
    let ccd = Ccd::enabled();
    let acceleration = Acceleration::default();
    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas.add(atlas),
        sprite: TextureAtlasSprite {
            index: 0,
            ..default()
        },
        ..default()
    };

    //TODO: Look into how to make constant - https://trello.com/c/bb7mUk8C/10-look-into-how-to-make-constant
    let body = Collider::capsule(Vec2::new(-6.0, 0.0), Vec2::new(8.0, 0.0), 11.0);
    let pos = TransformBundle::from(Transform::from_xyz(120.0, 100.0, 0.0));

    // Create Player character
    commands
        .spawn(body_type)
        .insert(body)
        .insert(name)
        .insert(pos)
        .insert(velocity)
        .insert(gravity)
        .insert(ccd)
        .insert(sprite)
        .insert(acceleration)
        .insert(trail_timer)
        .insert(Player);
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Velocity, &mut Acceleration)>,
) {
    for (_, mut velocity, mut acceleration) in query.iter_mut() {
        // Initialize variables
        let mut new_velocity = velocity.linvel;
        let mut new_acceleration = Vec2::ZERO;

        // Get user input
        if keyboard_input.pressed(KEY_UP) && !keyboard_input.pressed(KEY_DOWN) {
            new_acceleration += Vec2::Y * MOVEMENT_SPEED_MULTIPLIER;
        } else if keyboard_input.pressed(KEY_DOWN) && !keyboard_input.pressed(KEY_UP) {
            new_acceleration -= Vec2::Y * MOVEMENT_SPEED_MULTIPLIER;
        }
        if keyboard_input.pressed(KEY_LEFT) && !keyboard_input.pressed(KEY_RIGHT) {
            new_acceleration -= Vec2::X * MOVEMENT_SPEED_MULTIPLIER;
        } else if keyboard_input.pressed(KEY_RIGHT) && !keyboard_input.pressed(KEY_LEFT) {
            new_acceleration += Vec2::X * MOVEMENT_SPEED_MULTIPLIER;
        }

        // Apply friction
        new_velocity *= FRICITON;

        // Apply acceleration
        new_velocity += new_acceleration * time.delta_seconds();

        // Clamp velocity
        new_velocity = new_velocity.clamp_length(MIN_SPEED, MAX_SPEED);

        // Update velocity
        velocity.linvel = new_velocity;

        // Update acceleration
        acceleration.acc = new_acceleration;
    }
}

fn rotation(
    mut query: Query<(
        &Player,
        &mut Velocity,
        &mut Transform,
        &mut TextureAtlasSprite,
    )>,
) {
    for (_, velocity, mut transform, mut sprite) in query.iter_mut() {
        // Get the direction of the velocity
        let direction = velocity.linvel.normalize_or_zero();
        let left_or_right = if direction.x > 0.0 { 1 } else { -1 };

        // Calculate rotation
        let rotation = Quat::from_rotation_z(-(left_or_right as f32 * direction.y).acos())
            // Rotate 90 degrees to face the right direction
            * Quat::from_rotation_z(left_or_right as f32 * std::f32::consts::FRAC_PI_2);

        // Update transform
        transform.rotation = rotation;

        // Update sprite
        sprite.flip_y = left_or_right == -1;
    }
}

//TODO: Clean up trail - https://trello.com/c/vkhgB7yi/7-clean-up-trail
fn trail(
    time: Res<Time>,
    texture_atlas: Res<TextureAtlasResource>,
    mut commands: Commands,
    mut query: Query<(&Player, &Transform, &Acceleration, &Velocity, &mut Trail)>,
) {
    for (_, transform, acceleration, velocity, mut trail) in query.iter_mut() {
        trail.timer.tick(time.delta());
        if !trail.timer.finished() || acceleration.acc == Vec2::ZERO {
            continue;
        }

        // Create bubble
        let bubble_type = BubbleType::random();
        let bubble = Bubble::new(TRAIL_LIFETIME);

        // Create sprite
        let sprite = SpriteSheetBundle {
            texture_atlas: texture_atlas.handle.clone(),
            sprite: TextureAtlasSprite {
                index: bubble_type as usize,
                ..default()
            },
            ..default()
        };

        let name = Name::new(TRAIL_NAME);

        let mut velocity = Velocity {
            linvel: velocity.linvel * TRAIL_VELOCITY_MULTIPLIER,
            ..default()
        };

        // Spread bubbles
        velocity.linvel += Vec2::new(
            rand::thread_rng().gen_range(TRAIL_RANDOM_VELOCITY_MIN..TRAIL_RANDOM_VELOCITY_MAX),
            rand::thread_rng().gen_range(TRAIL_RANDOM_VELOCITY_MIN..TRAIL_RANDOM_VELOCITY_MAX),
        );

        let mut transform = transform.clone();
        transform.translation += transform.rotation * Vec3::new(-TEXTURE_SIZE.x / 1.5, 0.0, 0.0);

        // Spawn bubble
        commands
            .spawn(sprite)
            .insert(bubble)
            .insert(velocity.clone())
            .insert(name)
            .insert(transform)
            .insert(velocity);
    }
}
