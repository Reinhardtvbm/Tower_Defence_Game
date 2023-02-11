use bevy::prelude::{Component, Entity};

#[derive(Debug)]
pub enum TowerEntity {
    Tower(Tower),
    Enemy(Enemy),
    Path(Path),
}

#[derive(Debug, Component)]
pub struct Tower {
    health: f32,
    projectile_damage: f32,
    fire_rate: f32,
    entity: Entity,
}

impl Tower {
    pub fn new(health: f32, projectile_damage: f32, fire_rate: f32, entity: Entity) -> Self {
        Self {
            health,
            projectile_damage,
            fire_rate,
            entity,
        }
    }

    pub fn get_entity(&self) -> Entity {
        self.entity
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
}

#[derive(Debug)]
pub struct Enemy {
    health: usize,
    speed: usize,
}

impl Enemy {}

#[derive(Debug)]
pub struct Path;
