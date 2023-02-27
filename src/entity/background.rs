use bevy::prelude::*;

// Background plugin
pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_background);
    }
}

fn spawn_background(mut commands: Commands) {
    let size = 50.0; // adjust the size of the squares here
    let rows = 15;
    let cols = 25;
    let center = 1;

    for row in 0..=rows {
        for col in 0..=cols {
            let x = col as f32 * size - ((cols as f32 * size) / 2.0);
            let y = row as f32 * size - ((rows as f32 * size) / 2.0);
            let color;
            if row >= (rows / 2) - center
                && row <= (rows / 2) + center
                && col >= (cols / 2) - center
                && col <= (cols / 2) + center
            {
                if (row + col) % 2 == 0 {
                    color = Color::rgb(0.0, 0.0, 0.0);
                } else {
                    color = Color::rgb(1.0, 1.0, 1.0);
                }
            } else {
                if (row + col) % 2 == 0 {
                    color = Color::rgb(0.5, 0.0, 0.5);
                } else {
                    color = Color::rgb(1.0, 0.0, 1.0);
                }
            }

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    rect: Some(Rect::new(x, y, x + size, y + size)),
                    color,
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..Default::default()
            });
        }
    }
}
