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

1. Finish "Graphcis" outlined functions
    1. Add Write Text function (need a font??)

Research/Thinking Tasks:
1. Mouse/Cursor API? for UI + (mouse = right stick) emulation? How to handle networking here?
1. Brainstorm "UserApi" for stuff like player names, avatar, meta-data outside the game
1. Full screen shaders: Bloom, scanlines, etc

How to build WASM Projects:
cargo build --release --target wasm32-unknown-unknown

If WGPU errors occur, set WGPU_BACKEND=gl via

> export WGPU_BACKEND=gl

Useful Links:
Circle Drawing Algorithm
http://rosettacode.org/wiki/Bitmap/Midpoint_circle_algorithm


OPL Programming:
http://map.grauw.nl/resources/sound/yamaha_opl4.pdf
https://www.fit.vutbr.cz/~arnost/opl/opl3.html
http://jp.oplx.com/opl2.htm
https://doomwiki.org/wiki/OPL_emulation

Showcase Projects TODO:
Controller Debug - Showcase all controls for local player
Blasters - Twin stick shooter, Showcase two analog stick usage

