use crate::actions::Actions;
use crate::config::*;
use crate::loading::TextureAssets;
use crate::player::PlayerControlled;
use crate::GameState;
use bevy::prelude::*;
use rand::Rng;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_enemy.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn spawn_enemy(mut commands: Commands, textures: Res<TextureAssets>) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        let x: f32 = rng.gen_range(-200.0..200.0);
        let y: f32 = rng.gen_range(-200.0..200.0);

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
            .insert(PlayerControlled {
                velocity: Vec3::ZERO,
            });
    }
}
