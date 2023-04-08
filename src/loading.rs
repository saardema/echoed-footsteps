use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::LdtkAsset;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, LdtkLevelAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct LdtkLevelAssets {
    #[asset(path = "levels/level02.ldtk")]
    pub level: Handle<LdtkAsset>,
}
#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/footstep_01.ogg")]
    pub footstep_01: Handle<AudioSource>,
    #[asset(path = "audio/footstep_02.ogg")]
    pub footstep_02: Handle<AudioSource>,
    #[asset(path = "audio/footstep_03.ogg")]
    pub footstep_03: Handle<AudioSource>,
    #[asset(path = "audio/hit_wall.wav")]
    pub hit_wall: Handle<AudioSource>,
    #[asset(path = "audio/laser_shoot.wav")]
    pub laser_shoot: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/footstep.png")]
    pub footstep: Handle<Image>,
}
