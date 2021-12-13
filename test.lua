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
    local scale = 50
    clear_screen()
    set_pixel(x + math.sin(COUNT) * scale, y + math.cos(COUNT) * scale, PRESSES % 16)
end

