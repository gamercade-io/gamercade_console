COUNT = 0
PRESSES = 0

function init() 
    X_MID = width() / 2
    Y_MID = height() / 2
    SCALE = 80
    X_POS = X_MID
    Y_POS = Y_MID
end

function update(inputs)
    COUNT = COUNT + 0.0025

    for index, input in ipairs(inputs) do
        if (input.buttons.a) then
            PRESSES = PRESSES + 1
        end
    
        if (input.buttons.up) then
            Y_POS = Y_POS - 1
        end
    
        if (input.buttons.down) then
            Y_POS = Y_POS + 1
        end
    
        if (input.buttons.left) then
            X_POS = X_POS - 1
        end
    
        if (input.buttons.right) then
            X_POS = X_POS + 1
        end
    end
end

function draw()

    clear_screen()

    for i = 1, 15, 1 do
        local new_x = X_MID + math.sin(COUNT * i) * SCALE
        local new_y = Y_MID + math.cos(COUNT * i) * SCALE
        line(X_POS, Y_POS, new_x, new_y, (PRESSES + i) % 16)
    end

    rect(X_MID - SCALE, Y_MID - SCALE, SCALE * 2, SCALE * 2, (PRESSES + 5) % 16)
end

