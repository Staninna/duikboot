use super::data::{KeyInput, UserInput};
use crate::settings::player::{KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use bevy::prelude::*;
use ggrs::PlayerHandle;

// Needs to have the same logic as all systems that use input (keyboard_input: Res<Input<KeyCode>>)
pub fn input(_handle: In<PlayerHandle>, keyboard_input: Res<Input<KeyCode>>) -> UserInput {
    // Get user input
    let mut input = KeyInput::empty();
    if keyboard_input.pressed(KEY_UP) && !keyboard_input.pressed(KEY_DOWN) {
        input.insert(KeyInput::UP);
    } else if keyboard_input.pressed(KEY_DOWN) && !keyboard_input.pressed(KEY_UP) {
        input.insert(KeyInput::DOWN);
    }
    if keyboard_input.pressed(KEY_LEFT) && !keyboard_input.pressed(KEY_RIGHT) {
        input.insert(KeyInput::LEFT);
    } else if keyboard_input.pressed(KEY_RIGHT) && !keyboard_input.pressed(KEY_LEFT) {
        input.insert(KeyInput::RIGHT);
    }

    // Return input
    UserInput { input }
}
