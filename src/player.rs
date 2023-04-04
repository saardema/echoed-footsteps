use crate::actions::Actions;
use crate::config::*;
use crate::environment::*;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerControlled {
    pub velocity: Vec3,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_systems((move_player, update_camera).in_set(OnUpdate(GameState::Playing)));
    }
}

fn update_camera(mut query: Query<&mut Transform, With<Camera>>) {
    let mut transform = query.single_mut();

    *transform = Transform {
        translation: Vec3 {
            x: 0.,
            y: 0.,
            z: 999.,
        },
        scale: Vec3 {
            x: 0.7,
            y: 0.7,
            z: 1.,
        },
        ..default()
    };
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::splat(UNIT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 30.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(PlayerControlled {
            velocity: Vec3::ZERO,
        });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_controlled_query: Query<(&mut Transform, &mut PlayerControlled)>,
    wall_query: Query<(&Transform, &Wall), Without<PlayerControlled>>,
) {
    let mut input = Vec3::ZERO;
    let mut acceleration = DECELERATION;

    if actions.player_movement.is_some() {
        acceleration = ACCELERATION;
        input = Vec3::new(
            actions.player_movement.unwrap().x * SPEED * time.delta_seconds(),
            actions.player_movement.unwrap().y * SPEED * time.delta_seconds(),
            0.,
        );
    }

    for (mut transform, mut player_controlled) in player_controlled_query.iter_mut() {
        let velocity_difference = input - player_controlled.velocity;
        player_controlled.velocity += velocity_difference * acceleration;

        for (wall_transform, wall) in wall_query.iter() {
            if let Some(collision) = collide(
                transform.translation + player_controlled.velocity,
                Vec2::splat(UNIT),
                wall_transform.translation,
                wall.size,
            ) {
                match collision {
                    Collision::Left => {
                        if player_controlled.velocity.x > 0. {
                            player_controlled.velocity.x = (wall_transform.translation.x
                                - wall.size.x / 2.)
                                - (transform.translation.x + UNIT / 2.);
                        }
                    }
                    Collision::Right => {
                        if player_controlled.velocity.x < 0. {
                            player_controlled.velocity.x = (wall.size.x / 2.
                                + wall_transform.translation.x)
                                - (transform.translation.x - UNIT / 2.);
                        }
                    }
                    Collision::Top => {
                        if player_controlled.velocity.y < 0. {
                            player_controlled.velocity.y = (wall.size.y / 2.
                                + wall_transform.translation.y)
                                - (transform.translation.y - UNIT / 2.);
                        }
                    }
                    Collision::Bottom => {
                        if player_controlled.velocity.y > 0. {
                            player_controlled.velocity.y = (wall_transform.translation.y
                                - wall.size.y / 2.)
                                - (transform.translation.y + UNIT / 2.);
                        }
                    }
                    _ => {}
                }
            }
        }

        transform.translation += player_controlled.velocity;
    }
}
