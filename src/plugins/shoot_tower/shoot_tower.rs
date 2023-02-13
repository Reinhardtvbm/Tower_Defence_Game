use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    plugins::spawn_enemy::spawn_enemy::{Enemy, EnemyTranslations},
    resources::entity::{Tower, TowerEntity},
};

pub struct ShootTowerPlugin;

impl Plugin for ShootTowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shoot);
    }
}

fn shoot(
    mut commands: Commands,
    mut tower: Query<(&mut Transform, &mut TowerEntity)>,
    enemy_translations: Res<EnemyTranslations>,
    windows: Res<Windows>,
) {
    for (mut tower_transform, tower) in &mut tower {
        if let Some(enemy_pos) = enemy_translations.iter().last() {
            let (tower_x, tower_y) = (tower_transform.translation.x, tower_transform.translation.y);
            let (enemy_x, enemy_y) = (enemy_pos.x, enemy_pos.y);

            let tower_angle = (tower_y - enemy_y).atan2(tower_x - enemy_x) - (PI / 2.0);

            tower_transform.rotation = Quat::from_rotation_z(tower_angle);

            // println!(
            //     "atan2(({} - {}) / ({} - {})) = {}",
            //     tower_y,
            //     enemy_y,
            //     tower_x,
            //     enemy_x,
            //     tower_angle / (PI / 180.0)
            // );
        }
    }
}
