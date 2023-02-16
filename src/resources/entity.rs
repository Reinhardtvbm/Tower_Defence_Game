use bevy::prelude::Component;

use crate::plugins::spawn_enemy_plugin::components::enemy::Enemy;

#[derive(Debug, Component, Clone, Copy)]
pub enum TowerEntity {
    Tower(Tower),
    Enemy(Enemy),
    Path(Path),
}

#[derive(Debug, Component, Clone, Copy)]
pub struct Tower {
    health: f32,
    cost: f32,
    projectile_damage: f32,
    fire_rate: f32,
    range: f32,
}

impl Tower {
    pub fn new(health: f32, projectile_damage: f32, fire_rate: f32, range: f32, cost: f32) -> Self {
        Self {
            health,
            projectile_damage,
            fire_rate,
            range,
            cost
        }
    }

    pub fn cost(&self) -> f32 {
        self.cost
    }

    pub fn increase_damage_constant(&mut self, constant: f32) {
        self.projectile_damage += constant;
    }

    pub fn increase_damage_percentage(&mut self, percentage: f32) {
        self.projectile_damage *= 1.0 + (percentage / 100.0);
    }

    pub fn increase_fire_rate_constant(&mut self, constant: f32) {
        self.fire_rate += constant;
    }

    pub fn increase_fire_rate_percentage(&mut self, percentage: f32) {
        self.fire_rate *= 1.0 + (percentage / 100.0);
    }

    pub fn increase_health_constant(&mut self, constant: f32) {
        self.health += constant;
    }

    pub fn increase_health_percentage(&mut self, percentage: f32) {
        self.health *= 1.0 + (percentage / 100.0);
    }

    pub fn increase_range_constant(&mut self, constant: f32) {
        self.range += constant;
    }

    pub fn increase_range_percentage(&mut self, percentage: f32) {
        self.range *= 1.0 + (percentage / 100.0);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Path;
