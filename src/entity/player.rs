// TODO: Remove magic numbers

use crate::{
    components::acceleration::Acceleration,
    entity::bubble::Bubble,
    resource::texture::TextureAtlasResource,
    settings::{
        bubble::NAME as BUBBLE_NAME,
        player::{
            FRICITON, GRAVITY_SCALE, MAX_SPEED, MOVEMENT_SPEED_MULTIPLIER, NAME, TEXTURE,
            TEXTURE_SIZE,
        },
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

// TODO: Clean up this mess
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
) {
    // Create components
    let name = Name::new(NAME);
    let gravity = GravityScale(GRAVITY_SCALE);
    let body_type = RigidBody::Dynamic;
    let velocity = Velocity::zero();
    let ccd = Ccd::enabled();
    let acceleration = Acceleration::default();

    // TODO
    let body = Collider::capsule(Vec2::new(-6.0, 0.0), Vec2::new(8.0, 0.0), 11.0);
    let pos = TransformBundle::from(Transform::from_xyz(120.0, 100.0, 0.0));
    let trail_timer = Trail {
        timer: Timer::from_seconds(0.08, TimerMode::Repeating),
    };

    // Create sprite
    let atlas = TextureAtlas::from_grid(asset_server.load(TEXTURE), TEXTURE_SIZE, 1, 1, None, None);
    let sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas.add(atlas),
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
        .insert(ccd)
        .insert(sprite)
        .insert(acceleration)
        .insert(trail_timer)
        .insert(Player);

    // Create trail
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
        if keyboard_input.pressed(KeyCode::W) && !keyboard_input.pressed(KeyCode::S) {
            new_acceleration += Vec2::Y * MOVEMENT_SPEED_MULTIPLIER;
        } else if keyboard_input.pressed(KeyCode::S) && !keyboard_input.pressed(KeyCode::W) {
            new_acceleration -= Vec2::Y * MOVEMENT_SPEED_MULTIPLIER;
        }
        if keyboard_input.pressed(KeyCode::A) && !keyboard_input.pressed(KeyCode::D) {
            new_acceleration -= Vec2::X * MOVEMENT_SPEED_MULTIPLIER;
        } else if keyboard_input.pressed(KeyCode::D) && !keyboard_input.pressed(KeyCode::A) {
            new_acceleration += Vec2::X * MOVEMENT_SPEED_MULTIPLIER;
        }

        // Apply friction
        new_velocity *= FRICITON;

        // Apply acceleration
        new_velocity += new_acceleration * time.delta_seconds();

        // Clamp velocity
        new_velocity = new_velocity.clamp_length(0.0, MAX_SPEED);

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

        // Smooth rotation
        transform.rotation = transform.rotation.slerp(rotation, 0.1);

        // Update transform
        transform.rotation = rotation;

        // Update sprite
        sprite.flip_y = left_or_right == -1;
    }
}

// TODO: Clean up this mess
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
        let bubble = Bubble::new(1.5);

        // Create sprite
        let sprite = SpriteSheetBundle {
            texture_atlas: texture_atlas.handle.clone(),
            sprite: TextureAtlasSprite {
                index: bubble_type as usize,
                ..default()
            },
            ..default()
        };

        let name = Name::new(BUBBLE_NAME);

        let mut velocity = Velocity {
            linvel: velocity.linvel * -0.1,
            ..default()
        };

        // Spread bubbles
        velocity.linvel += Vec2::new(
            rand::thread_rng().gen_range(-6.0..6.0),
            rand::thread_rng().gen_range(-6.0..6.0),
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
