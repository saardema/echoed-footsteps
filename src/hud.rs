use bevy::prelude::*;

use crate::{
    config::{WINDOW_HEIGHT, WINDOW_WIDTH},
    player::PlayerState,
    GameState,
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init)
            .add_system(update.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
struct HealthBarFront;

#[derive(Component)]
struct HealthBarBack;

fn init(mut commands: Commands) {
    commands.spawn((
        HealthBarFront,
        SpriteBundle {
            sprite: Sprite {
                color: Color::GRAY,
                custom_size: Some(Vec2::new(400., 10.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                WINDOW_WIDTH / 2.,
                WINDOW_HEIGHT - 10.,
                100.,
            )),
            ..default()
        },
    ));

    commands.spawn((
        HealthBarFront,
        SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(400., 10.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                WINDOW_WIDTH / 2.,
                WINDOW_HEIGHT - 10.,
                105.,
            )),
            ..default()
        },
    ));
}

fn update(player_state: Res<PlayerState>, mut bar: Query<&mut Sprite, With<HealthBarFront>>) {
    for sprite in bar.iter_mut() {
        if let Some(mut size) = sprite.custom_size {
            size.x = player_state.hp;
        }
    }
}
