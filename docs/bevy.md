## Bevy Resources

A resource is a **unique** struct that can be accessed globally by _all systems_.

```Rust
#[derive(Resource)]
struct MyResource(usize)
```
