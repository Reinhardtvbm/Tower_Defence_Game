#[derive(Debug)]
pub enum TowerEntity {
    DefenceTower(DefenceTower),
    Enemy(Enemy),
    Path(Path),
}

#[derive(Debug)]
pub struct DefenceTower;

impl DefenceTower {}

#[derive(Debug)]
pub struct Enemy;

impl Enemy {}

#[derive(Debug)]
pub struct Path;

impl Path {}
