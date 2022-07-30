use crate::{loading::TextureAssets, GameState};

use bevy::{prelude::*, sprite::Anchor};
use bevy_pixel_camera::{PixelBorderPlugin, PixelCameraBundle, PixelCameraPlugin};

pub struct PixelViewPlugin;

impl Plugin for PixelViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PixelCameraPlugin)
            .add_plugin(PixelBorderPlugin {
                color: Color::BLACK,
            })
            .add_system_set(
                SystemSet::on_enter(GameState::Menu)
                    .with_system(setup_pixel_camera)
                    .with_system(setup_pixel_grid),
            );
    }
}

fn setup_pixel_camera(mut commands: Commands) {
    commands.spawn_bundle(PixelCameraBundle::from_resolution(64, 64));
}

fn setup_pixel_grid(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(SpriteBundle {
        texture: textures.texture_grid.clone(),
        sprite: Sprite {
            anchor: Anchor::BottomLeft,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-32.0, -32.0, 0.0),
            ..default()
        },
        ..default()
    });
}
