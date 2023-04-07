use std::time::Duration;

use crate::actions::Actions;
use crate::components::*;
use crate::config::*;
use crate::enemy::EnemyBundle;
use crate::environment::Goal;
use crate::environment::Wall;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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
            .add_system(spawn_ldtk_entities)
            .add_systems((footsteps, update_velocity, rotate).in_set(OnUpdate(GameState::Playing)));
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
    fn new(position: Vec3) -> Self {
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
        let interval = FOOTSTEP_INTERVAL * (10. - player_speed).max(0.11);
        timer.0.set_duration(Duration::from_secs_f32(interval));
        timer.0.tick(time.delta());

        // Footstep age
        for (entity, mut footstep, mut sprite) in &mut footstep_query {
            footstep.age += time.delta_seconds();

            sprite.color.set_a(1. - footstep.age / footstep.max_age);

            if footstep.age >= footstep.max_age {
                commands.entity(entity).despawn();
            }
        }

        // Spawn new footsteps
        if player_speed > 0.1 && timer.0.just_finished() {
            events.send(FootstepEvent);

            player.used_left_foot = !player.used_left_foot;

            let mut transform = player_transform.clone();
            transform.translation.z -= 1.;
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
                        color: Color::rgb(0.8, 0.8, 1.0),
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

fn spawn_ldtk_entities(
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
) {
    for (entity, transform, entity_instance) in entity_query.iter() {
        let mut position = transform.translation.clone();

        if entity_instance.identifier == *"PlayerSpawner" {
            position.z = 30.;
            commands.spawn(PlayerBundle::new(position));
        } else if entity_instance.identifier == *"EnemySpawner" {
            position.z = 20.;
            commands.spawn(EnemyBundle::new(position));
        } else if entity_instance.identifier == *"Goal" {
            position.z = 10.;
            commands.spawn((
                Goal,
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR1,
                        custom_size: Some(Vec2::splat(UNIT)),
                        ..default()
                    },
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
            ));
        } else if entity_instance.identifier == *"WallSpawner" {
            let size = Vec2::new(transform.scale.x, transform.scale.y) * UNIT;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR8,
                        custom_size: Some(size),
                        ..default()
                    },
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
                Wall,
                StaticCollider { size },
            ));
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
