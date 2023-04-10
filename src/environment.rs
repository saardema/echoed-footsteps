use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_ecs_ldtk::prelude::*;

use crate::components::*;
use crate::config::*;
use crate::enemy::EnemyBundle;
use crate::loading::LdtkLevelAssets;
use crate::player::PlayerBundle;
use crate::GameState;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::Index(0))
            .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.1)))
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .add_system(spawn_ldtk_entities)
            .add_system(setup_level.in_schedule(OnEnter(GameState::Playing)));
    }
}
#[derive(Component, Reflect, Debug)]
pub struct Wall;

#[derive(Component)]
pub struct Goal;

#[derive(Bundle)]
pub struct WallBundle {
    wall: Wall,
    collider: StaticCollider,
    sprite_bundle: SpriteBundle,
}

impl WallBundle {
    pub fn new(position: Vec3, size: Vec2) -> Self {
        Self {
            wall: Wall,
            collider: StaticCollider { size },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: COLOR7,
                    custom_size: Some(size),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..Default::default()
            },
        }
    }
}

fn setup_level(
    mut commands: Commands,
    level_assets: Res<LdtkLevelAssets>,
    level_query: Query<&LevelSet>,
) {
    if !level_query.is_empty() {
        return;
    }

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.level.clone(),
        ..default()
    });

    // Bottom
    commands.spawn(WallBundle::new(
        Vec3::new(WINDOW_WIDTH / 2. + UNIT, UNIT / 2., 1.),
        Vec2::new(WINDOW_WIDTH, UNIT),
    ));

    // Top
    commands.spawn(WallBundle::new(
        Vec3::new(WINDOW_WIDTH / 2. + UNIT, WINDOW_HEIGHT + UNIT * 1.5, 1.),
        Vec2::new(WINDOW_WIDTH, UNIT),
    ));

    // Left
    commands.spawn(WallBundle::new(
        Vec3::new(UNIT / 2., WINDOW_HEIGHT / 2. + UNIT, 1.),
        Vec2::new(UNIT, WINDOW_HEIGHT),
    ));

    // Right
    commands.spawn(WallBundle::new(
        Vec3::new(WINDOW_WIDTH + UNIT * 1.5, WINDOW_HEIGHT / 2. + UNIT, 1.),
        Vec2::new(UNIT, WINDOW_HEIGHT),
    ));
}

fn spawn_ldtk_entities(
    mut commands: Commands,
    entity_query: Query<(Entity, &Transform, &EntityInstance), Added<EntityInstance>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, transform, entity_instance) in entity_query.iter() {
        let mut position = transform.translation.clone();

        if entity_instance.identifier == *"PlayerSpawner" {
            position.z = 30.;
            commands.entity(entity).insert(PlayerBundle::new(position));
        } else if entity_instance.identifier == *"EnemySpawner" {
            position.z = 20.;
            commands.entity(entity).insert(EnemyBundle::new(position));
        } else if entity_instance.identifier == *"Goal" {
            position.z = 10.;
            commands.entity(entity).insert((
                Goal,
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(UNIT * 0.7).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    transform: Transform::from_translation(position),
                    ..default()
                },
            ));
        } else if entity_instance.identifier == *"WallSpawner" {
            position.z = 5.;
            let size = Vec2::new(transform.scale.x, transform.scale.y) * UNIT;
            commands
                .entity(entity)
                .insert(WallBundle::new(position, size));
        }
    }
}
