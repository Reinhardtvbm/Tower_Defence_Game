use bevy::prelude::Resource;

#[derive(Resource)]
pub struct PlaceFSM {
    state: PlaceState,
}

impl PlaceFSM {
    pub fn new() -> Self {
        Self {
            state: PlaceState::Idle,
        }
    }

    pub fn toggle(&mut self) {
        self.state = match self.state {
            PlaceState::Place => PlaceState::Idle,
            PlaceState::Idle => PlaceState::Place,
        }
    }

    pub fn get_state(&self) -> PlaceState {
        self.state
    }
}

#[derive(Clone, Copy)]
pub enum PlaceState {
    Place,
    Idle,
}
