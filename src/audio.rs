use std::time::Duration;

use crate::actions::{set_movement_actions, Actions};
use crate::components::*;
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system(start_audio.in_schedule(OnEnter(GameState::Playing)))
            .add_system(
                control_footstep_sound
                    .after(set_movement_actions)
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
struct FootstepsAudio(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.pause();
    let handle = audio
        .play(audio_assets.footsteps.clone())
        .start_from(3.8)
        .handle();
    commands.insert_resource(FootstepsAudio(handle));
}

fn control_footstep_sound(
    footsteps_audio: Res<FootstepsAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut player_velocity_query: Query<&Velocity, With<Player>>,
) {
    return;
    if let Some(footsteps_instance) = audio_instances.get_mut(&footsteps_audio.0) {
        let player_velocity = player_velocity_query.single_mut();
        let volume = (player_velocity.0.length() / 30.).min(1.);

        footsteps_instance.set_volume((volume + 0.2) as f64, AudioTween::default());
        footsteps_instance.resume(AudioTween::default());

        if volume < 0.01 {
            footsteps_instance.pause(AudioTween::default());
        } else {
            footsteps_instance.resume(AudioTween::default());
            if footsteps_instance.state().position() > Some(3.8) {
                footsteps_instance.seek_to(0.65);
            }
        }
    }
}
