# Vulkys

An easy-to-use but extensive physics engine meant to be accessible to those with little to no programming experience.

## Requirements

1. Vulkan (tested v1.3.292)
2. [The Rust programming language](https://www.rust-lang.org/) (stable)

`cargo r` should run fine. Do not run on release.

### Compatibility
- Windows: runtime error
- MacOS (MoltenVK): runtime error
- Ubuntu: Complete

## Commits

Although not uploaded to GitHub with the intention of maintenance or feature-requests, pull requests (PRs) are welcome.

## Goals
- [ ] Copy images into swapchain so the current, un-updated ones are not being recreated

### Performance
- [x] ~~Profile current program~~
- [x] ~~Have collision resolution run on a compute shader~~
- [ ] Add FPS counter for debugging and visuals
- [ ] Copy images into swapchain so the current, un-updated ones are not being recreated

### QoL
- [ ] Anti-aliasing for objects
