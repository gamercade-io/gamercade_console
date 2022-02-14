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


--- INPUT ---
Each input call requires a player_id;

Button examples:
get_a_held(1) - returns true if player 1 is holding A
get_b_pressed(2) - returns true if player 2 has just pressed B this frame
get_start_released(3) - returns true if player 3 just released start this frame

Valid Buttons:
up
down
left
right
a
b
c
d
start
select
left_shoulder
right_shoulder
left_stick
right_stick
left_trigger
right_trigger

Analogs: have an x and y axis. values returned will be from 0 (center) to 1 (max movement)
get_left_analog_x(1) - returns a value from 0 to 1 for player 1's left analog x-axis.
get_left_analog_y(2) - returns a value from 0 to 1 for player 2's left analog y-axis.
get_right_analog_x(1) - returns a value from 0 to 1 for player 1's right analog x-axis.
get_right_analog_y(3) - returns a value from 0 to 1 for player 3's right analog y-axis.

Triggers: returns values from 0 (completely released) to 1 (completely depressed)
get_left_trigger(1) - returns a value from 0 to 1 for player 1's left trigger
get_right_trigger(2) - returns a value from 0 to 1 for player 2's right trigger


--- DRAWING ---
clear_screen(optional color_index, optional palette_index)
set_pixel(x, y, optional color_index, optional palette_index)
height()
width()
rect(x, y, width, height, optional color_index, optional palette_index)
line(x0, y0, x1, y1, optional color_index, optional palette_index)

```


TODO:
1. Do "Input State" and reading / gamepads
    1. Do "Emulated gamepad" for keyboard input, include analogs etc
1. Incorporate networking functionality
1. Fix issues when trying to draw out of bounds
1. Write "Pong" in test.lua
1. Finish "Graphcis" outlined functions
    1. Add "Draw Square" filled?
    1. Add "Draw Circle"
    1. Add "Draw Circle" filled?
    1. Add Write Text function
1. Brainstorm "UserApi" for stuff like player names, avatar, meta-data outside the game
1. Set the user count accessable as a global somewhere

If WGPU errors occur, set WGPU_BACKEND=gl via

> export WGPU_BACKEND=gl

Useful Links:
Circle Drawing Algorithm
http://rosettacode.org/wiki/Bitmap/Midpoint_circle_algorithm

Showcase Projects TODO:
Pong 1p - Simple showcase
Pong 2p - Showcase networking multiplayer
Controller Debug - Showcase all controls for local player
Blasters - Twin stick shooter, Showcase two analog stick usage
