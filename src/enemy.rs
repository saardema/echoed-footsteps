use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

use crate::components::*;
use crate::config::*;
use crate::environment::Wall;
use crate::loading::AudioAssets;
use crate::player::Player;
use crate::player::PlayerVelocityHistory;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_kira_audio::Audio;
use bevy_kira_audio::AudioControl;
use rand::Rng;

pub struct EnemyPlugin;

#[derive(Component, Default)]
pub struct Enemy {
    shoot_timer: Timer,
    offset: usize,
    can_see_player: bool,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                update_enemy_velocity,
                rotate_enemy,
                projectile_hit,
                shoot,
                update_projectiles,
            )
                .in_set(OnUpdate(GameState::Playing)),
        );
    }
}

#[derive(Component)]
struct Projectile {
    direction: Vec3,
    speed: f32,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    velocity: Velocity,
    collider: DynamicCollider,
    sprite_bundle: SpriteBundle,
}

impl EnemyBundle {
    pub fn new(position: Vec3) -> Self {
        let mut rng = rand::thread_rng();
        let mut shoot_timer = Timer::new(
            Duration::from_secs_f32(rng.gen_range(2.8..3.2)),
            TimerMode::Repeating,
        );
        shoot_timer.set_elapsed(Duration::from_secs_f32(rng.gen_range(0.0..1.0)));

        Self {
            enemy: Enemy {
                shoot_timer,
                offset: rng.gen_range(0..50),
                can_see_player: true,
            },
            velocity: Velocity::default(),
            collider: DynamicCollider {
                size: Vec2::splat(UNIT),
            },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR6,
                    custom_size: Some(Vec2::new(UNIT, UNIT / 2.)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..Default::default()
            },
        }
    }
}

fn update_enemy_velocity(
    mut enemy_velocity_query: Query<(&mut Velocity, &Enemy)>,
    mut player_velocity_history: ResMut<PlayerVelocityHistory>,
) {
    for (mut enemy_velocity, enemy) in enemy_velocity_query.iter_mut() {
        enemy_velocity.0 = player_velocity_history.get(enemy.offset);
    }
}

fn rotate_enemy(mut enemy_query: Query<(&mut Transform, &Velocity), With<Enemy>>) {
    for (mut transform, velocity) in enemy_query.iter_mut() {
        if velocity.0.length() > 0. {
            transform.rotation = Quat::from_rotation_arc(Vec3::Y, velocity.0.normalize());
        }
    }
}

fn update_projectiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Projectile)>,
    time: Res<Time>,
) {
    for (entity, mut transform, projectile) in query.iter_mut() {
        transform.translation += projectile.direction * projectile.speed * time.delta_seconds();

        if transform.translation.x > WINDOW_WIDTH
            || transform.translation.x < 0.
            || transform.translation.y > WINDOW_HEIGHT
            || transform.translation.y < 0.
        {
            commands.entity(entity).despawn();
        }
    }
}

fn shoot(
    mut commands: Commands,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (transform, mut enemy) in enemy_query.iter_mut() {
            enemy.shoot_timer.tick(time.delta());

            if enemy.shoot_timer.just_finished() && enemy.can_see_player {
                audio
                    .play(audio_assets.laser_shoot.clone())
                    .with_volume(0.1);

                commands.spawn((
                    Projectile {
                        direction: (player_transform.translation - transform.translation)
                            .normalize(),
                        speed: 300.,
                    },
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::YELLOW,
                            custom_size: Some(Vec2::splat(3.)),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3 {
                            x: transform.translation.x,
                            y: transform.translation.y,
                            z: transform.translation.z - 1.,
                        }),
                        ..Default::default()
                    },
                    DynamicCollider {
                        size: Vec2::splat(2.),
                    },
                ));
            }
        }
    }
}

fn projectile_hit(
    mut commands: Commands,
    walls: Query<(&StaticCollider, &Transform), With<Wall>>,
    projectiles: Query<(Entity, &DynamicCollider, &Transform), With<Projectile>>,
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
) {
    for (w_collider, w_transform) in walls.iter() {
        for (entity, p_collider, p_transform) in projectiles.iter() {
            let collision = collide(
                w_transform.translation,
                w_collider.size,
                p_transform.translation,
                p_collider.size,
            );

            if collision.is_some() {
                audio.play(audio_assets.hit_wall.clone()).with_volume(0.1);
                commands.entity(entity).despawn();
            }
        }
    }
}
