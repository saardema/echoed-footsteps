use crate::loading::AudioAssets;
use crate::player::{FootstepEvent, Player};
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system(on_footstep.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Resource)]
struct FootstepsAudio(Handle<AudioInstance>);

fn on_footstep(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut events: EventReader<FootstepEvent>,
    player_query: Query<&Player>,
) {
    for _ in events.iter() {
        let Ok(player) = player_query.get_single() else {return;};

        if player.used_left_foot {
            audio
                .play(audio_assets.footstep_03.clone())
                .with_volume(0.7);
        } else {
            audio
                .play(audio_assets.footstep_01.clone())
                .with_volume(0.7);
        }
    }
}
