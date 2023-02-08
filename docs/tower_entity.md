# Tower Entity struct

```Rust
pub enum TowerEntity {
    DefenceTower(DefenceTower),
    Enemy(Enemy),
    Path(Path),
}
```

The TowerEntity enum can represent a number of possible game entities:

- A Defense Tower
- An enemy
- Path (that enemies travel along)
- maybe more...
