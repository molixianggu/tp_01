#![windows_subsystem = "windows"]
mod game;

use bevy::{asset::AssetServerSettings, prelude::*, render::texture::ImageSettings};
use bevy_embedded_assets::EmbeddedAssetPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
    });
    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            // width: 800.0,
            // height: 480.0,
            title: "命运召唤 尔茄的精灵石".to_string(),
            canvas: Some("#main".to_string()),
            fit_canvas_to_parent: true,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugin(game::GamePlugin);

    app.insert_resource(AssetServerSettings {
        watch_for_changes: true,
        ..default()
    });

    app.run();
}
