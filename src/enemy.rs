use crate::components::*;
use crate::config::*;
use crate::player::PlayerVelocityHistory;
use crate::GameState;
use bevy::prelude::*;
use rand::Rng;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct DelayedPlayerController;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemy.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update_enemy_velocity.in_set(OnUpdate(GameState::Playing)));
    }
}

fn spawn_enemy(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x: f32 = rng.gen_range(-80.0..80.0);
        let y: f32 = rng.gen_range(-80.0..80.0);

        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::splat(UNIT)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 20.)),
                ..Default::default()
            })
            .insert(Enemy)
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
