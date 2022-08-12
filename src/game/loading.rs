use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::{GameState, libs::{AnimationData, AnimationLoader, Animation}};

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<AnimationLoader>();
        app.add_asset::<AnimationData>();
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::Start)
            .with_collection::<AnimationAssets>()
            .with_collection::<TextureAssets>()
            .init_resource::<Animation>()
        );
    }
}


#[derive(AssetCollection)]
pub struct AnimationAssets {
    #[asset(path = "animation/player01.anim_ske.json")]
    pub player01: Handle<AnimationData>,
}


#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "tiled/textures/780.jpg")]
    pub bg: Handle<Image>,
}

