pub enum TowerEntity {
    DefenceTower(DefenceTower),
    Enemy(Enemy),
    Path(Path),
}

pub struct DefenceTower;

impl DefenceTower {}

pub struct Enemy;

impl Enemy {}

pub struct Path;

impl Path {}
