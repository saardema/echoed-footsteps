use std::time::Duration;

use crate::actions::Actions;
use crate::components::*;
use crate::config::*;
use crate::environment::Goal;
use crate::loading::AudioAssets;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioControl;

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FootstepEvent>()
            .insert_resource(PlayerState::default())
            .insert_resource(FootstepTimer(Timer::new(
                Duration::from_secs_f32(FOOTSTEP_INTERVAL),
                TimerMode::Repeating,
            )))
            .insert_resource(PlayerVelocityHistory::new(50))
            .add_systems(
                (footsteps, update_velocity, rotate, level_complete)
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Component, Default)]
pub struct Player {
    pub used_left_foot: bool,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    velocity: Velocity,
    collider: DynamicCollider,
    sprite_bundle: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(position: Vec3) -> Self {
        Self {
            player: Player {
                used_left_foot: false,
            },
            velocity: Velocity::default(),
            collider: DynamicCollider {
                size: Vec2::splat(UNIT),
            },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR5,
                    custom_size: Some(Vec2::new(UNIT, UNIT / 2.)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..Default::default()
            },
        }
    }
}

#[derive(Resource, Default)]
pub struct PlayerState {
    pub hp: f32,
}

#[derive(Resource)]
struct FootstepTimer(Timer);

#[derive(Component, Reflect)]
struct Footstep {
    age: f32,
    max_age: f32,
}

pub struct FootstepEvent;

#[derive(Resource)]
pub struct PlayerVelocityHistory {
    velocities: Vec<Vec3>,
    size: usize,
    pointer: usize,
}

impl PlayerVelocityHistory {
    fn new(size: usize) -> Self {
        Self {
            velocities: vec![Vec3::ZERO; size],
            size: size,
            pointer: 0,
        }
    }

    pub fn get(&mut self, offset: usize) -> Vec3 {
        self.velocities[(self.pointer + 1 + offset) % self.size]
    }

    fn set(&mut self, velocity: Vec3) {
        self.velocities[self.pointer] = velocity;
        self.pointer = (self.pointer + 1) % self.size;
    }
}

fn footsteps(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform, &Velocity)>,
    mut footstep_query: Query<(Entity, &mut Footstep, &mut Sprite)>,
    mut timer: ResMut<FootstepTimer>,
    mut events: EventWriter<FootstepEvent>,
    textures: Res<TextureAssets>,
    time: Res<Time>,
) {
    if let Ok((mut player, player_transform, player_velocity)) = player_query.get_single_mut() {
        let player_speed = player_velocity.0.length();

        // Distance between footsteps
        let interval = FOOTSTEP_INTERVAL * (15. - player_speed).max(0.11);
        timer.0.set_duration(Duration::from_secs_f32(interval));
        timer.0.tick(time.delta());

        // Footstep age
        for (entity, mut footstep, mut sprite) in &mut footstep_query {
            footstep.age += time.delta_seconds();

            sprite
                .color
                .set_a(0.5 - footstep.age / footstep.max_age / 2.);

            if footstep.age >= footstep.max_age {
                commands.entity(entity).despawn();
            }
        }

        // Spawn new footsteps
        if player_speed > 2.0 && timer.0.just_finished() {
            events.send(FootstepEvent);

            player.used_left_foot = !player.used_left_foot;

            let mut transform = player_transform.clone();
            transform.translation.z = 5.;
            transform.translation +=
                transform.local_x() * (if player.used_left_foot { 5. } else { -5. });

            commands.spawn((
                Footstep {
                    age: 0.,
                    max_age: FOOTSTEP_MAX_AGE,
                },
                SpriteBundle {
                    texture: textures.footstep.clone(),
                    sprite: Sprite {
                        color: COLOR5,
                        flip_x: !player.used_left_foot,
                        custom_size: Some(Vec2::splat(20.)),
                        ..default()
                    },
                    transform,
                    ..Default::default()
                },
            ));
        }
    }
}

fn level_complete(
    goal_query: Query<&Transform, With<Goal>>,
    mut player_query: Query<(&Transform, &mut Velocity), With<Player>>,
    mut state: ResMut<NextState<GameState>>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    mut player_velocity_history: ResMut<PlayerVelocityHistory>,
) {
    if let Ok((player, mut velocity)) = player_query.get_single_mut() {
        if let Ok(goal) = goal_query.get_single() {
            let distance = goal.translation - player.translation;

            if distance.length() < UNIT * 2. {
                velocity.0 = Vec3::ZERO;
                *player_velocity_history = PlayerVelocityHistory::new(50);
                state.set(GameState::LevelComplete);

                audio
                    .play(audio_assets.level_complete.clone())
                    .with_volume(0.2);
            }
        }
    }
}

fn update_velocity(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_velocity_history: ResMut<PlayerVelocityHistory>,
    mut player_velocity_query: Query<&mut Velocity, With<Player>>,
) {
    let mut input = Vec3::ZERO;
    let mut acceleration = DECELERATION;

    if actions.player_movement.is_some() {
        acceleration = ACCELERATION;
        input = Vec3::new(
            actions.player_movement.unwrap().x * SPEED * time.delta_seconds(),
            actions.player_movement.unwrap().y * SPEED * time.delta_seconds(),
            0.,
        );
    }

    if let Ok(mut player_velocity) = player_velocity_query.get_single_mut() {
        let velocity_difference = input - player_velocity.0;
        player_velocity.0 += velocity_difference * acceleration * time.delta_seconds();

        player_velocity_history.set(player_velocity.0);
    }
}

fn rotate(mut player_query: Query<(&mut Transform, &Velocity), With<Player>>) {
    if let Ok((mut transform, velocity)) = player_query.get_single_mut() {
        if velocity.0.length() > 0. {
            transform.rotation = Quat::from_rotation_arc(Vec3::Y, velocity.0.normalize());
        }
    }
}
