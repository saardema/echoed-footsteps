use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_ecs_ldtk::prelude::*;

use crate::components::StaticCollider;
use crate::config::*;
use crate::enemy::EnemyBundle;
use crate::loading::LdtkLevelAssets;
use crate::player::PlayerBundle;
use crate::GameState;

pub struct EnvironmentPlugin;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Goal;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::Index(1))
            .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.1)))
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .add_system(spawn_ldtk_entities)
            .add_system(setup_level.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn setup_level(mut commands: Commands, level_assets: Res<LdtkLevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.level.clone(),
        ..default()
    });
}

fn spawn_ldtk_entities(
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (_, transform, entity_instance) in entity_query.iter() {
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
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(UNIT * 0.7).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    transform: Transform::from_translation(position),
                    ..default()
                },
                // SpriteBundle {
                //     sprite: Sprite {
                //         color: Color::GREEN,
                //         custom_size: Some(Vec2::splat(UNIT)),
                //         ..default()
                //     },
                //     transform: Transform::from_translation(position),
                //     ..Default::default()
                // },
            ));
        } else if entity_instance.identifier == *"WallSpawner" {
            position.z = 5.;
            let size = Vec2::new(transform.scale.x, transform.scale.y) * UNIT;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: COLOR7,
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
