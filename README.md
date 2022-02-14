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
Required Functions
init() - Called once when initializing the game

update(inputs) - Called once every frame, before draw. Inputs parameter is a table containing all inputs from all players for this frame.

draw() - Called once every frame, after update.


About inputs...
An array from indicies 1-4 for each connected player. Can access various inputs like
inputs[1].buttons.a - gets the 1st player's A button
inputs[3].buttons.up - gets the 3rd player's up button

Complete list can be found below...
buttons:
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
lshoulder
rshoulder
rstick
lstick
ltrigger
rtrigger

triggers:
TODO

analogs:
TODO
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

If WGPU errors occur, set WGPU_BACKEND=gl via

> export WGPU_BACKEND=gl

Useful Links:
Circle Drawing Algorithm
http://rosettacode.org/wiki/Bitmap/Midpoint_circle_algorithm