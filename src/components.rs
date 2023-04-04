use bevy::prelude::*;

#[derive(Component)]
pub struct StaticCollider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct DynamicCollider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Player;
