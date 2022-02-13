COUNT = 0
PRESSES = 0

function input(id, pressed)
    if (pressed) then
        PRESSES = PRESSES + 1
    end

end

function update()
    COUNT = COUNT + 0.005
end

function draw()
    local x = width() / 2
    local y = height() / 2
    local scale = 75
    clear_screen()
    local new_x = x + math.sin(COUNT) * scale
    local new_y = y + math.cos(COUNT) * scale
    -- set_pixel(x + math.sin(COUNT) * scale, y + math.cos(COUNT) * scale, PRESSES % 16)
    line(x, y, new_x, new_y, PRESSES % 16)
end

