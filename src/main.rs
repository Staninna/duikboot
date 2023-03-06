use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod entity;
mod network;
mod resource;
mod settings;

fn main() {
    // Create game
    let mut game = App::new();

    // Initialize networking
    network::init::init_network(&mut game);

    // Add default plugins
    game.add_plugins(DefaultPlugins);

    // Add pysics engine
    game.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

    // Add plugins
    game.add_plugin(entity::camera::CameraPlugin)
        .add_plugin(entity::player::PlayerPlugin)
        .add_plugin(entity::bubble::BubblePlugin);

    // Add debug plugins if debug build
    #[cfg(debug_assertions)]
    game.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(entity::background::BackgroundPlugin) // Temporary background for debugging
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
        .add_plugin(bevy_rapier2d::render::RapierDebugRenderPlugin::default())
        .add_plugin(bevy_inspector_egui_rapier::InspectableRapierPlugin)
        .add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin);

    // Run game
    game.run();
}
