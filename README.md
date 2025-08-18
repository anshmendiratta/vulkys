# Vulkys

An easy-to-use but extensive physics engine meant to be accessible to those with little to no programming experience.

## Requirements

1. Vulkan (tested v1.3.292)
2. [The Rust programming language](https://www.rust-lang.org/) (stable)

## Usage

1. Create objects in `main.rs`.
2. Create a scene with said objects in a `Vec<RigidBody>` using `Scene::with_objects`.
3. Create an `App` with `App::with_scene(scene)`.
4. Create an `EventLoop`.
5. Run the app using `event_loop.run_app(&mut app)`.
6. `cargo r` should run fine.

Example mains in `examples/`.

### Compatibility
- Linux:
  - Mint (Sway): Complete.
  - Ubuntu (WSL): Complete.
- Windows: image format runtime error.
- MacOS: Complete.

## Commits

Although not uploaded to GitHub with the intention of maintenance or feature-requests, pull requests (PRs) and issues are welcome.

## Goals
- [ ] Document more thoroughly.
- [x] Add new Rapier3D object types.
- [x] Add shading so objects such as balls do not need further detailing.

### Performance
- [ ] Add FPS counter for debugging and visuals.
- [ ] Add multi-threading.

### QoL
- [ ] Anti-aliasing for objects.
- [ ] Add a global directional light source (sun) and shadows.
- [ ] Try to add materials/finishes (diffuse/glossy/metallic).
- [ ] Use textures for background and objects.
