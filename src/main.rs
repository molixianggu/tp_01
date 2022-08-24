// #![windows_subsystem = "windows"]
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

mod test_op {
    #[test]
    fn test_exp() {
        use evalexpr::ContextWithMutableVariables;

        let compiled = evalexpr::build_operator_tree("velocity_y < 0").unwrap();
        let mut ctx = evalexpr::HashMapContext::new();

        ctx.set_value("velocity_y".into(), (-0.4).into()).unwrap();
        let result = compiled.eval_with_context(&ctx).unwrap();
        println!("result = {:?}", result);
    }
}
