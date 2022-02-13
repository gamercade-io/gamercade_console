COUNT = 0
PRESSES = 0

function input(id, pressed)
    if (pressed) then
        PRESSES = PRESSES + 1
    end

end

function update()
    COUNT = COUNT + 0.0025
end

function draw()
    local x_mid = width() / 2
    local y_mid = height() / 2
    local scale = 80
    clear_screen()

    -- set_pixel(x + math.sin(COUNT) * scale, y + math.cos(COUNT) * scale, PRESSES % 16)

    for i = 1, 15, 1 do
        local new_x = x_mid + math.sin(COUNT * i) * scale
        local new_y = y_mid + math.cos(COUNT * i) * scale
        line(x_mid, y_mid, new_x, new_y, (PRESSES + i) % 16)
    end

    rect(x_mid - scale, y_mid - scale, scale * 2, scale * 2, (PRESSES + 5) % 16)
end

