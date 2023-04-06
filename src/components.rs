use bevy::prelude::*;

use crate::config::UNIT;

#[derive(Component)]
pub struct StaticCollider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct DynamicCollider {
    pub size: Vec2,
}

impl Default for DynamicCollider {
    fn default() -> Self {
        Self {
            size: Vec2::splat(UNIT),
        }
    }
}

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);
