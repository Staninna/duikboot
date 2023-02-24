//TODO: Add comments - https://trello.com/c/8HXAWr31/9-add-comments

use crate::{
    components::acceleration::Acceleration,
    entity::bubble::{Bubble, BubbleType},
    resource::texture::TextureAtlasResource,
    settings::player::{
        COLLIDER_SHAPE_A, COLLIDER_SHAPE_B, COLLIDER_SHAPE_RADIUS, FRICITON, GRAVITY_SCALE,
        KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP, MAX_SPEED, MIN_SPEED, MOVEMENT_SPEED_MULTIPLIER,
        NAME, START_POSITION, TEXTURE, TEXTURE_OFFSET, TEXTURE_PADDING, TEXTURE_SHEET_SIZE,
        TEXTURE_SIZE, TRAIL_LIFETIME, TRAIL_NAME, TRAIL_RANDOM_VELOCITY_MAX,
        TRAIL_RANDOM_VELOCITY_MIN, TRAIL_TICK, TRAIL_VELOCITY_MULTIPLIER,
    },
};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

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
    let body = Collider::capsule(COLLIDER_SHAPE_A, COLLIDER_SHAPE_B, COLLIDER_SHAPE_RADIUS);
    let pos = START_POSITION;
    let texture = TextureAtlas::from_grid(
        asset_server.load(TEXTURE),
        TEXTURE_SIZE,
        TEXTURE_SHEET_SIZE[0],
        TEXTURE_SHEET_SIZE[1],
        TEXTURE_PADDING,
        TEXTURE_OFFSET,
    );

    // Defined in this file (won't change)
    let body_type = RigidBody::Dynamic;
    let velocity = Velocity::zero();
    let acceleration = Acceleration::default();
    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas.add(texture),
        sprite: TextureAtlasSprite {
            index: 0,
            ..default()
        },
        ..default()
    };

    // Create Player character
    commands
        .spawn(body_type)
        .insert(body)
        .insert(name)
        .insert(pos)
        .insert(velocity)
        .insert(gravity)
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

fn trail(
    time: Res<Time>,
    texture_atlas: Res<TextureAtlasResource>,
    mut commands: Commands,
    mut query: Query<(&Player, &Transform, &Acceleration, &Velocity, &mut Trail)>,
) {
    for (_, transform, acceleration, velocity, mut trail) in query.iter_mut() {
        // Skip when acceleration is zero or timer is not finished
        trail.timer.tick(time.delta());
        if !trail.timer.finished() || acceleration.acc == Vec2::ZERO {
            continue;
        }

        // Create bubble
        let bubble = Bubble::new(TRAIL_LIFETIME);
        let name = Name::new(TRAIL_NAME);
        let mut velocity = velocity.clone();
        let mut transform = transform.clone();

        // Create sprite
        let bubble_type = BubbleType::random() as usize;
        let sprite = SpriteSheetBundle {
            texture_atlas: texture_atlas.handle.clone(),
            sprite: TextureAtlasSprite {
                index: bubble_type,
                ..default()
            },
            ..default()
        };

        // Set velocity
        velocity.linvel *= TRAIL_VELOCITY_MULTIPLIER;

        // Spread bubbles
        velocity.linvel += Vec2::new(
            rand::thread_rng().gen_range(TRAIL_RANDOM_VELOCITY_MIN..=TRAIL_RANDOM_VELOCITY_MAX),
            rand::thread_rng().gen_range(TRAIL_RANDOM_VELOCITY_MIN..=TRAIL_RANDOM_VELOCITY_MAX),
        );

        // Set position to the back of the player
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
