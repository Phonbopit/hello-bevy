Hello Bevy
---

## Notes

- `add_system` is run in parellel.
- Plugins - `UiPlugin`, `RenderPlugin`
- `add_plugins(DefaultPlugins)` - will popup a window.
- `DefaultPlugins` - add an **event loop** to application. (per frame)

```rust
add_plugins(DefaultPlugins)
```

Equal to

```rust
.add_plugin(CorePlugin::default())
.add_plugin(InputPlugin::default())
.add_plugin(WindowPlugin::default())
```
