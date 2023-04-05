use crate::loading::AudioAssets;
use crate::player::FootstepEvent;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::Rng;

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
) {
    for _ in events.iter() {
        let choice = rand::thread_rng().gen_range(1..=3);
        let mut handle = audio_assets.footstep_01.clone();

        if choice == 2 {
            handle = audio_assets.footstep_02.clone();
        } else if choice == 3 {
            handle = audio_assets.footstep_03.clone();
        }

        audio.play(handle).with_volume(0.3);
    }
}
