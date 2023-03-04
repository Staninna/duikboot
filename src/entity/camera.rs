use bevy::prelude::*;

use super::player::Player;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera).add_system(move_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn move_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    for mut camera_transform in camera.iter_mut() {
        for player_transform in player.iter() {
            camera_transform.translation = player_transform.translation;
        }
    }
}
