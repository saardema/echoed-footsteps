use crate::actions::Actions;
use crate::components::*;
use crate::config::*;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerVelocityHistory::new(50))
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update_camera.in_schedule(OnEnter(GameState::Playing)))
            .add_systems(
                (update_player_velocity, update_camera, rotate_player)
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Resource)]
pub struct PlayerVelocityHistory {
    velocities: Vec<Vec3>,
    size: usize,
    pointer: usize,
}

impl PlayerVelocityHistory {
    fn new(size: usize) -> Self {
        Self {
            velocities: vec![Vec3::ZERO; size],
            size: size,
            pointer: 0,
        }
    }

    pub fn get(&mut self) -> Vec3 {
        self.velocities[(self.pointer + 1) % self.size]
    }

    fn set(&mut self, velocity: Vec3) {
        self.velocities[self.pointer] = velocity;
        self.pointer = (self.pointer + 1) % self.size;
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

fn spawn_player(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(UNIT, UNIT / 2.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 30.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(Vec3::ZERO))
        .insert(DynamicCollider {
            size: Vec2::splat(UNIT),
        });
}

fn update_player_velocity(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_velocity_history: ResMut<PlayerVelocityHistory>,
    mut player_velocity_query: Query<&mut Velocity, With<Player>>,
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

    let mut player_velocity = player_velocity_query.single_mut();
    let velocity_difference = input - player_velocity.0;
    player_velocity.0 += velocity_difference * acceleration * time.delta_seconds();

    player_velocity_history.set(player_velocity.0);
}

fn rotate_player(mut player_query: Query<(&mut Transform, &Velocity), With<Player>>) {
    if let Ok((mut transform, velocity)) = player_query.get_single_mut() {
        transform.rotation =
            Quat::from_euler(EulerRot::XYZ, 0., 0., -(velocity.0.x / velocity.0.y).atan());
    }
}
