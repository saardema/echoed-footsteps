use bevy::prelude::*;

use crate::GameState;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component)]
struct HealthBar {
    background_sprite_bundle: SpriteBundle,
    foreground_sprite_bundle: SpriteBundle,
}

fn init(mut commands: Commands) {
    commands.spawn(
        (HealthBar {
            background_sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::GRAY,
                    custom_size: Some(Vec2::new(400., 10.)),
                    ..default()
                },
                ..default()
            },
            foreground_sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(400., 10.)),
                    ..default()
                },
                ..default()
            },
        }),
    );
}

fn update() {}
