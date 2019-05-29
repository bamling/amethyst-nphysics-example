# amethyst-nphysics-example

Simple example *"game"* illustrating how an integration of [nphysics](https://www.nphysics.org/)  into [Amethyst](https://amethyst.rs/) could look like. This example uses the `master` version of Amethyst and has all the nphysics related logic abstracted away in a separate crate (`game_physics`).

Rust version:
`rustc 1.36.0-nightly (3991285f5 2019-04-25)`

#### Running the example:

Clone the repository:
```bash
$ git clone https://github.com/bamling/amethyst-nphysics-example.git
```

Change into the checkout directory:
```bash
$ cd amethyst-nphysics-example
```

Execute [Cargo](https://doc.rust-lang.org/cargo/):
```bash
$ cargo run
```

#### Roadmap:

- [x] Allow multiple `PhysicsCollider`s per `Entity`
- [x] Allow `PhysicsCollider`s without `PhysicsBody`
- [x] Automatically apply margin to full `Shape` size
- [ ] Add more `Shape`s
- [ ] Add other debug shapes
- [ ] Expose channels for `CollisionEvent`s and `ProximityEvent`s 
- [x] Remove custom `Isometry`, `Matrix` and `Point` types
- [ ] Refactor body/collider `Sytem`s
- [ ] Ray interferences to prevent tunneling issues*
- [ ] Custom `GameData` with separate dispatcher for movement/physics based `System`s (executed during `fixed_update(..)`)
- [x] Clean up `game_physics` crate exports
- [ ] Add tests
- [ ] Introduce generic type parameters over `f32`
- [ ] Examples on how to use the crate
- [ ] Polishing, polishing, polishing...
- [ ] Publish to [crates.io](https://crates.io) (migrate repository for that?)



\*Thanks to [sebcrozet](https://github.com/sebcrozet) for the idea:
> Well, you can cast a ray on the world using: https://www.nphysics.org/rustdoc/nphysics3d/world/struct.ColliderWorld.html#method.interferences_with_ray. The ray direction would be the player desired velocity, and the ray starting point would be right in front of the player in that direction. If interferences are found, then find the one with the smallest toi field: https://www.ncollide.org/rustdoc/ncollide3d/query/ray_internal/struct.RayIntersection.html#structfield.toi which is < 1.0. Then you take this toi (if it exists) and multiply it with the players' desired velocity. This will give you a new vector that is the velocity you actually want to apply to your player.

```rust
let point_in_front_of_the_player = desired_velocity.normalize() * (player_box_radius + 0.1) + player_center_position;
let ray = Ray::new(point_in_front_of_the_player, desired_velocity);
let toi = world.interferences_with_ray(ray, CollisionGroups::default()).fold(1.0, |a, inter| a.min(inter.1.toi));
let velocity_to_apply = desired_velocity * toi;
```