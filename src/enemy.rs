use std::time::Duration;

use crate::components::*;
use crate::config::*;
use crate::player::PlayerVelocityHistory;
use crate::GameState;
use bevy::prelude::*;
use rand::Rng;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy {
    timer: Timer,
}

#[derive(Component)]
pub struct DelayedPlayerController;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemy.in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (
                    update_enemy_velocity,
                    rotate_enemy,
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

fn spawn_enemy(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x: f32 = rng.gen_range(-180.0..180.0);
        let y: f32 = rng.gen_range(-180.0..180.0);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(UNIT, UNIT / 2.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 20.)),
                ..Default::default()
            })
            .insert(Enemy {
                timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating),
            })
            .insert(Velocity(Vec3::ZERO))
            .insert(DynamicCollider {
                size: Vec2::splat(UNIT),
            })
            .insert(DelayedPlayerController);
    }
}

fn update_enemy_velocity(
    mut enemy_velocity_query: Query<&mut Velocity, (With<Enemy>, Without<Player>)>,
    mut player_velocity_history: ResMut<PlayerVelocityHistory>,
) {
    for mut enemy_velocity in enemy_velocity_query.iter_mut() {
        enemy_velocity.0 = player_velocity_history.get();
    }
}

fn rotate_enemy(mut enemy_query: Query<(&mut Transform, &Velocity), With<Enemy>>) {
    for (mut transform, velocity) in enemy_query.iter_mut() {
        transform.rotation =
            Quat::from_euler(EulerRot::XYZ, 0., 0., -(velocity.0.x / velocity.0.y).atan());
    }
}

fn update_projectiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Projectile)>,
    time: Res<Time>,
) {
    for (entity, mut transform, projectile) in query.iter_mut() {
        transform.translation += projectile.direction * projectile.speed * time.delta_seconds();

        if transform.translation.x > WINDOW_WIDTH / 2.
            || transform.translation.x < -WINDOW_WIDTH / 2.
            || transform.translation.y > WINDOW_HEIGHT / 2.
            || transform.translation.y < -WINDOW_HEIGHT / 2.
        {
            commands.entity(entity).despawn();
        }
    }
}

fn shoot(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Enemy)>,
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = player_query.single();
    for (transform, mut enemy) in query.iter_mut() {
        enemy.timer.tick(time.delta());

        if enemy.timer.just_finished() {
            commands.spawn((
                Projectile {
                    direction: (player_transform.translation - transform.translation).normalize(),
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
            ));
        }
    }
}
