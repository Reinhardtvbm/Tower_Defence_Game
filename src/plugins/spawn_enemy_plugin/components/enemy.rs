use bevy::prelude::Component;

#[derive(Debug, Clone, Component, Copy)]
pub struct Enemy {
    health: Health,
    speed: f32,
    id: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Health {
    full: f32,
    actual: f32,
}

impl Health {
    pub fn new(full: f32) -> Self {
        Self { full, actual: full }
    }

    pub fn full(&self) -> f32 {
        self.full
    }

    pub fn actual(&self) -> f32 {
        self.actual
    }
}

impl Enemy {
    pub fn new(health: f32, speed: f32, id: usize) -> Self {
        Self {
            health: Health::new(health),
            speed,
            id,
        }
    }

    pub fn current_health(&self) -> f32 {
        self.health.actual()
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
