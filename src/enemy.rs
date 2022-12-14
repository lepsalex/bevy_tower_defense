use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    common::{Health, Target},
    navigation::NavAgent,
    physics::PhysicsBundle,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EnemyType>()
            .register_type::<EnemyBundle>();
    }
}

#[derive(Reflect, Component, Clone, Copy, Default)]
#[reflect(Component)]
pub enum EnemyType {
    #[default]
    EnemyBasic,
}

#[derive(Default, Bundle, Reflect)]
pub struct EnemyBundle {
    target: Target,
    health: Health,
    nav_agent: NavAgent,
    #[reflect(ignore)]
    physics_bundle: PhysicsBundle,
}

impl EnemyBundle {
    pub fn new(
        health: i32,
        move_speed: f32,
        turn_speed: f32,
        route: Vec<Vec3>,
        physics_bundle: PhysicsBundle,
    ) -> Self {
        Self {
            target: Target,
            health: Health { value: health },
            nav_agent: NavAgent {
                move_speed,
                turn_speed,
                delay_timer: Timer::from_seconds(0.5, TimerMode::Once),
                route,
                ..default()
            },
            physics_bundle,
        }
    }
}

pub fn spawn_enemy(
    commands: &mut Commands,
    location: Vec3,
    enemy_type: EnemyType,
    nav_route: &Vec<Vec3>,
    game_assets: &Res<GameAssets>,
) {
    match enemy_type {
        EnemyType::EnemyBasic => {
            commands.spawn((
                SceneBundle {
                    scene: game_assets.ufo_red_scene.clone(),
                    transform: Transform::from_translation(location)
                        .looking_at(*nav_route.last().unwrap(), Vec3::Y),
                    ..default()
                },
                EnemyBundle::new(
                    3,
                    1.2,
                    2.4,
                    nav_route.clone(),
                    PhysicsBundle::moving_entity_sphere(0.55),
                ),
                Name::new("Enemy (Basic)"),
                EnemyType::EnemyBasic,
            ));
        }
    }
}
