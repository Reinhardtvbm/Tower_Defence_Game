pub mod plugins {
    pub mod shoot_tower_plugin;
    pub mod spawn_enemy_plugin;

    pub mod gui {
        pub mod main_menu;
    }

    pub mod debug_grid {
        pub mod debug_grid;
    }

    pub mod place_tower {
        pub mod place_fsm;
        pub mod place_tower;
    }
}

pub mod resources {
    pub mod entity;
    pub mod grid_map;
}
