# Dink

A test roguelike game using the [Bevy engine](https://bevyengine.org/).

## Running

If you don't have Rust, you can install it using [rustup](https://rustup.rs).
On Debian based systems you can install the required packages using `apt install libasound2-dev libudev-dev`.
Then run

```shell
cargo run
```

## Licencing

This project is released under the MIT Licence found in the [licence file](LICENCE), except for the files [`src/helpers/camera.rs`](src/helpers/camera.rs) and [`src/helpers/texture.rs`](src/helpers/texture.rs) which are from the[`bevy_ecs_tilemap` examples](https://github.com/StarArawn/bevy_ecs_tilemap/tree/main/examples/helpers) and are licenced under that [project's licence](https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/LICENSE).
