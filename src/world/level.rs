use bevy::prelude::*;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

use crate::{GameAssets, SpawnEntityMapping, SpawnPoint, TowerBaseLocation, Waypoint};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_level);
    }
}

// Names matching node name in Blender
const TOWER_BASE_LOCATION_NAME: &str = "grass";
const SPAWN_LOCATION_NAME: &str = "spawn";
const WAYPOINT_LOCATION_NAME: &str = "waypoint";

fn spawn_level(mut commands: Commands, game_assets: Res<GameAssets>) {
    // Spawn Level
    commands
        .spawn_bundle(HookedSceneBundle {
            scene: SceneBundle {
                scene: game_assets.level_0.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            hook: SceneHook::new(|entity, cmds| {
                entity.get::<Name>().map(|name| {
                    /*
                    Attach required components for marked tiles
                    */
                    if name.starts_with(TOWER_BASE_LOCATION_NAME) {
                        cmds.insert(TowerBaseLocation);
                    }

                    if name.starts_with(SPAWN_LOCATION_NAME) {
                        let data: Vec<&str> = name.split(".").collect();
                        cmds.insert(SpawnPoint {
                            id: data.get(1).unwrap().parse::<u32>().unwrap(),
                            spawn_entity: SpawnEntityMapping::EnemyBasic,
                            spawn_timer: Timer::from_seconds(3.0, true),
                            max_spawns: 3,
                            ..default()
                        })
                        .insert(Name::new("SpawnPoint"));
                    }

                    if name.starts_with(WAYPOINT_LOCATION_NAME) {
                        let data: Vec<&str> = name.split(".").collect();
                        cmds.insert(Waypoint {
                            id: data.get(2).unwrap().parse::<u32>().unwrap(),
                            spawn_id: data.get(1).unwrap().parse::<u32>().unwrap(),
                        });
                    }
                });
            }),
        })
        .insert(Name::new("Level"));
}
