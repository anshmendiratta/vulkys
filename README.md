# Vulkys

An easy-to-use but extensive physics engine meant to be accessible to those with little to no programming experience.



## Requirements

1. Vulkan (tested v1.3.292)
2. [The Rust programming language](https://www.rust-lang.org/) (stable)

## Usage

1. Create objects in `main.rs`. Create a scene with said objects in a `Vec<RigidBody>` using `Scene::with_objects` and then use `scene.run()`.
2. `cargo r` should run fine. Do not run on release.

Example mains in `examples/`.

### Compatibility
- Ubuntu (WSL): Complete.
- Windows: image format runtime error.
- MacOS (MoltenVK): seldom runtime errors.

## Commits

Although not uploaded to GitHub with the intention of maintenance or feature-requests, pull requests (PRs) and issues are welcome.

## Goals
- [ ] Copy images into swapchain so the current, un-updated ones are not being recreated.
- [ ] Document more thoroughly.
- [ ] Allow choice of timestep and gravity. Normalize these values to be somewhat intuitive.
- [ ] Add new object types.

### Performance
- [x] ~~Profile current program.~~
- [x] ~~Have collision resolution run on a compute shader.~~
- [ ] Add FPS counter for debugging and visuals.
- [ ] Copy images into swapchain so the current, un-updated ones are not being recreated.

### QoL
- [ ] Anti-aliasing for objects.
- [ ] Use textures for background and ojects.
