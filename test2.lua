COUNT = 0
PRESSES = 0
SPEED = 0.75

function init()
    X_MID = width() / 2
    Y_MID = height() / 2
    SCALE = 80
    X_POS = X_MID
    Y_POS = Y_MID
end

function update()
    COUNT = COUNT + 0.0025

    if (button_a_pressed(1)) then
        PRESSES = PRESSES + 1
    end

    if (button_up_held(1)) then
        Y_POS = Y_POS - SPEED
    end

    if (button_down_held(1)) then
        Y_POS = Y_POS + SPEED
    end

    if (button_left_held(2)) then
        X_POS = X_POS - SPEED
    end

    if (button_right_held(2)) then
        X_POS = X_POS + SPEED
    end
end

function draw()

    clear_screen()

    for i = 1, 4, 1 do
        local new_x = X_MID + math.sin(COUNT * i * 3) * SCALE
        local new_y = Y_MID + math.cos(COUNT * i * 3) * SCALE
        line(X_POS, Y_POS, new_x, new_y, (PRESSES + i) % 16)
    end

    rect(X_MID - SCALE, Y_MID - SCALE, SCALE * 2, SCALE * 2, (PRESSES + 5) % 16)
end

