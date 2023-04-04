use crate::components::*;
use crate::config::*;
use crate::GameState;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_and_collide.in_set(OnUpdate(GameState::Playing)));
    }
}

fn move_and_collide(
    static_collider_query: Query<(&Transform, &StaticCollider)>,
    mut dynamic_collider_query: Query<
        (&mut Transform, &mut Velocity, &DynamicCollider),
        Without<StaticCollider>,
    >,
) {
    for (mut dynamic_transform, velocity, dynamic_collider) in dynamic_collider_query.iter_mut() {
        let mut translation = velocity.0.clone();

        for (static_transform, static_collider) in static_collider_query.iter() {
            if let Some(collision) = collide(
                dynamic_transform.translation + velocity.0,
                dynamic_collider.size,
                static_transform.translation,
                static_collider.size,
            ) {
                match collision {
                    Collision::Left => {
                        if velocity.0.x > 0. {
                            translation.x = (static_transform.translation.x
                                - static_collider.size.x / 2.)
                                - (dynamic_transform.translation.x + UNIT / 2.);
                        }
                    }
                    Collision::Right => {
                        if velocity.0.x < 0. {
                            translation.x = (static_collider.size.x / 2.
                                + static_transform.translation.x)
                                - (dynamic_transform.translation.x - UNIT / 2.);
                        }
                    }
                    Collision::Top => {
                        if velocity.0.y < 0. {
                            translation.y = (static_collider.size.y / 2.
                                + static_transform.translation.y)
                                - (dynamic_transform.translation.y - UNIT / 2.);
                        }
                    }
                    Collision::Bottom => {
                        if velocity.0.y > 0. {
                            translation.y = (static_transform.translation.y
                                - static_collider.size.y / 2.)
                                - (dynamic_transform.translation.y + UNIT / 2.);
                        }
                    }
                    _ => {}
                }
            }
        }

        dynamic_transform.translation += translation;
    }
}
