use std::slice::Iter;

use bevy::prelude::*;

use crate::resources::grid_map::{grid::Grid, grid_coord::GridCoord};

pub struct SpawnEnemyPlugin;

impl Plugin for SpawnEnemyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            4.0,
            TimerMode::Repeating,
        )))
        .insert_resource(EnemyCount(0))
        .insert_resource(EnemyTranslations(Vec::new()))
        .add_system(spawn_enemies)
        .add_system(move_enemies);
    }
}

#[derive(Component)]
pub struct Enemy(usize);

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Resource)]
struct EnemyCount(usize);

#[derive(Resource, Debug)]
pub struct EnemyTranslations(Vec<Vec2>);

impl EnemyTranslations {
    pub fn push(&mut self, translation: Vec2) {
        self.0.push(translation);
    }

    pub fn iter(&self) -> Iter<'_, Vec2> {
        self.0.iter()
    }

    pub fn mutate_translation(&mut self, index: usize, new_val: Vec2) {
        self.0[index] = new_val;
    }
}

fn spawn_enemies(
    commands: Commands,
    grid: Res<Grid>,
    time: Res<Time>,
    mut enemy_translations: ResMut<EnemyTranslations>,
    mut count: ResMut<EnemyCount>,
    mut timer: ResMut<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        println!("1 seconds...");
        spawn_single_enemy(
            commands,
            asset_server,
            grid.get(&GridCoord(2, 2)).unwrap().get_spawn_position(),
            grid.get_cell_length(),
            count.0,
        );

        enemy_translations.push(Vec2::ZERO);

        count.0 += 1;
    }
}

fn move_enemies(
    time: Res<Time>,
    mut enemy_position: Query<(&mut Transform, &Enemy)>,
    mut enemy_translations: ResMut<EnemyTranslations>,
) {
    for (mut transform, enemy) in &mut enemy_position {
        transform.translation.x += 100. * time.delta_seconds();

        let new_translation = Vec2 {
            x: transform.translation.x,
            y: transform.translation.y,
        };

        enemy_translations.mutate_translation(enemy.0, new_translation);
    }
}

fn spawn_single_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    position: Vec2,
    cell_length: f32,
    id: usize,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: position.x,
                        y: position.y,
                        z: 0.0,
                    },
                    rotation: Quat::from_rotation_x(0.0),
                    scale: Vec3 {
                        x: cell_length / 100.0,
                        y: cell_length / 100.0,
                        z: 500.0,
                    },
                },
                texture: asset_server.load("test_enemy.png"),
                ..default()
            },
            Enemy(id),
        ))
        .id()
}
