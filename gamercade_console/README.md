# Gamercade Console

A Wasm powered Fantasy Console.

Learn more about [Gamercade](https://gamercade.io).

## Quick Start:

In order to write a game, you need expose these three functions in the language of your choice:

```
Required Functions:
init() - Called once when initializing the game
update() - Called once every frame, before draw.
draw() - Called once every frame, after update.
```

Then, compile it to Wasm. You must then bundle your game with any related art assets via the [Gamercade Editor](https://github.com/gamercade-io/gamercade_editor).

## Steps to Bundle & Export a game:

1. Open the Editor.
2. On the File menu, click "Select game .wasm." Find and locate your previously exported .wasm file.
3. On the File menu, click "Export game" and export.

You can now play the game by opening it up via the Console.

## Default Controls:

| **Control** | **Key** | &#124; | **Control** | **Key** | &#124; | **Control** | **Key** | &#124; | **Control** | **Key** | |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| _Left Analog Up_ | **W** | &#124; | _Right Analog Up_ | **T**| &#124; | _D-Pad Up_ | **Up Arrow** | &#124; | _A Button_ | **U** |
| _Left Analog Down_ | **S** | &#124; | _Right Analog Down_ | **G** | &#124; | _D-Pad Down_ | **Down Arrow** | &#124; | _B Button_ | **I** |
| _Left Analog Left_ | **A** | &#124; | _Right Analog Left_ | **F** | &#124; | _D-Pad Left_ | **Left Arrow** | &#124; | _C Button_ | **J** |
| _Left Analog Right_ | **D** | &#124; | _Right Analog Right_ | **H** | &#124; | _D-Pad Right_ | **Right Arrow** | &#124; | _D Button_ | **K** |
| _Left Analog L3 / Click_ | **X** | &#124; | _Right Analog R3 / Click_ | **B** | &#124;| _Start_ | **5** | &#124; | _Select_ | **6** |
| _Left Bumper (L1)_ | **E** | &#124; | _Left Analog (L2)_ | **Q** | &#124; | _Right Bumper (R1)_ | **R** | &#124; | _Right Analog (L2)_ | **Y** |

## WGPU Error

If WGPU errors occur, try to set `WGPU_BACKEND` environment variable to `gl` via (platform dependent):

> export WGPU_BACKEND=gl

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.