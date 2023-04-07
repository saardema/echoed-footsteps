use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::config::*;
use crate::loading::LdtkLevelAssets;
use crate::GameState;

pub struct EnvironmentPlugin;

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Goal;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::Index(1))
            .insert_resource(ClearColor(COLOR7))
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .add_system(setup_level.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn setup_level(mut commands: Commands, level_assets: Res<LdtkLevelAssets>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: level_assets.level.clone(),
        ..default()
    });
}
