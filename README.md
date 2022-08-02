# Gamercade Console

A wasm powered Fantasy Console.

Learn more about [Gamercade](https://gamercade.io), or head over to the the other related projects such as:

[Gamercade Console](https://github.com/gamercade-io/gamercade_console)

[Gamercade Editor](https://github.com/gamercade-io/gamercade_editor)

[Gamercade Core](https://github.com/gamercade-io/gamercade_core)

[Gamercade Site](https://github.com/gamercade-io/gamercade_site)

```
Keyboard Controls:
Left Analog Stick:
Up: W
Down: S
Left: A
Right: D
L3 / Click: X

Right Analog Stick:
Up: T
Down: G
Left: F
Right: H
R3 / Click : B

Left Bumper (L1): E
Left Analog (L2): Q

Right Bumper (R1): R
Right Analog (L2): Y

Digital Pad:
Up: Up Arrow
Down: Down Arrow
Left: Left Arrow
Right: Right Arrow

Buttons:
A Button: U
B Button: I
C Button: J
D Button: K
Start: 5
Select: 6
```

## Quick Start:
```
Required Functions:
init() - Called once when initializing the game

update() - Called once every frame, before draw.

draw() - Called once every frame, after update.

```

Potential Future Additions: 
1. Mouse/Cursor API? for UI with emulation.
1. "UserApi" for things like player names, avatar, meta-data outside the game.
1. Full screen shaders: Bloom, scanlines, etc

How to build WASM Projects:
cargo build --release --target wasm32-unknown-unknown

If WGPU errors occur, set WGPU_BACKEND=gl via

> export WGPU_BACKEND=gl

Showcase Projects TODOs:
Controller Debug - Showcase all controls for local player
Blasters - Twin stick shooter, Showcase two analog stick usage

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.