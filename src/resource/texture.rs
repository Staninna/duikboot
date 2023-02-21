use bevy::prelude::*;

#[derive(Resource)]
pub struct TextureAtlasResource {
    pub handle: Handle<TextureAtlas>,
}

impl TextureAtlasResource {
    pub fn new(handle: Handle<TextureAtlas>) -> Self {
        Self { handle }
    }
}
