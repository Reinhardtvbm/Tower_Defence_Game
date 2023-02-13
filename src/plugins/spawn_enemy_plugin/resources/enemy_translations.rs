use std::slice::Iter;

use bevy::prelude::{Resource, Vec2};

#[derive(Resource, Debug)]
pub struct EnemyTranslations(pub Vec<Vec2>);

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
