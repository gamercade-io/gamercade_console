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

TODO (In order of priority):
1. Networking Functionality (WASM):
    1. Test if input for prev and "was pressed" is actually correct handling is actually correct
    1. Check how increasing/decreasing heap mem works (will need another test suite)
1. Example games/wasm folder?
    1. Helper script / build script to automatically place into correct directory
1. Input handling
    1. Do "Emulated gamepad" for keyboard input, include analogs etc
    1. Gamepad support
    1. Support multiple local players
    1. Use SoAs instead of AoS for better perf?
1. Finish "Graphcis" outlined functions
    1. Add "Draw Square" filled?
    1. Add "Draw Circle"
    1. Add "Draw Circle" filled?
    1. Add Write Text function (need a font??)
    1. How to handle lines/rects drawn out of bounds?
1. Add deterministic random number generation function
    1. Use fastrand or nanorand crates
1. Build showcase projects
1. Start adding "Sprites" and Sprite Drawing
    1. Bit blit algorithm
1. Set the user count accessable as a global somewhere
1. Figure out how to save/load/rollback lua state?
    1. See the rlua comment about globals() vs _env
    1. https://github.com/amethyst/rlua/issues/247
1. Lua Refactoring
    1. Remove the inner arcs, use input/graphics context instead?

Research/Thinking Tasks:
1. Investigate how to play sounds
    1. Sound engine limitations, how many channels (16?), samples etc
    1. 44.1 khz sample rate, 16bit depth
    1. Support Sound Limits:
        1. 16 FM channels, 16 sample channels for music
        1. 16 FM channels, 16 sample channels for "game sounds"
    1. Support wave forms:
        1. Simple Sin (opl0) Done
        1. Half Sine (opl1) Done
        1. Absolute Sine (opl2) Done
        1. Quarter Sine (opl3) Done
        1. Alternating Sine (opl4) Done
        1. Camel Sine (opl5)
        1. Sharktooth ?? What is this ??
        1. Sawtooth Done
        1. Logarithmic Sawtooth (opl7) Done
        1. Square (opl6) Done
        1. Pulse Done
        1. Triangle Done
        1. Double Triangle ?? What is this ??
        1. Noise Done
    1. Support stereo sounds?
    1. Samples should be built in to the engine
    1. Select up to 512 possible samples
1. Mouse/Cursor API? for UI + (mouse = right stick) emulation? How to handle networking here?
1. Brainstorm "UserApi" for stuff like player names, avatar, meta-data outside the game
1. Cloud "Save/Load" Api - for game saves? Pulling session/game data?
1. Full screen shaders: Bloom, scanlines, etc



How to build WASM Projects:
cargo build --target wasm32-unknown-unknown

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
Pong 1p - Simple showcase
Pong 2p - Showcase networking multiplayer
Controller Debug - Showcase all controls for local player
Blasters - Twin stick shooter, Showcase two analog stick usage
