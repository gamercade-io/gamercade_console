Gamercade Editor

See Figma for UI design notes

rfd crate for file dialogs

TODO: Short Term:
- Add General Functionliaty
    - Editor JSON Save
    - Editor JSON Load
    - Export ROM
    - Import Sprite
- Palette Editor
    - Rename Palettes Button Functionality
- Sheet Editor
    - Adjustable/Changable Sprite Sheet Settings
    - New/Copy/Import/Edit/MoveLeftRight/Delete Sprites
    - Palette Previewer for entire sprite sheet
- Sprite Editor
    - Design TODO!
- Sound Editor
    - Incorporate Sound system
    - TODO!
- See other TODO's in the code

TODO: Long Term:
- How to handle localization?

Supported File Formats:
png: use image-png crate? has info struct and color_type, bit_depth, and palette fields
gif: use image-gif crate?
bmp
tiff - maybe not necessary??

... 

Or just use image-rs and parse each image manually into a palette & index data, failing on >16 colors