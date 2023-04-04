use bevy::math::Vec2Swizzles;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use rand::Rng;

use crate::config::*;
use crate::loading::LdtkLevelAssets;
use crate::GameState;

pub struct EnvironmentPlugin;

#[derive(Component)]
pub struct Wall {
    pub size: Vec2,
}

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Index(0))
            .add_system(setup_level.in_schedule(OnEnter(GameState::Playing)));
        // .add_system(spawn_walls.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn setup_level(mut commands: Commands, level_assets: Res<LdtkLevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.level01.clone(),
        ..default()
    });
}

fn spawn_walls(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    for _ in 0..15 {
        let position = Vec3::new(
            rng.gen_range(-15..15) as f32 * UNIT,
            rng.gen_range(-15..15) as f32 * UNIT,
            10.,
        );

        let mut size = Vec2::new(rng.gen_range(1..10) as f32 * UNIT, UNIT);

        if rng.gen::<f32>() > 0.5 {
            size = size.yx();
        }

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..Default::default()
            },
            Wall { size },
        ));
    }
}
