#![allow(unused)]

// Graphics
extern "C" {
    pub fn clear_screen(color_index: i32, palette_index: i32);
    pub fn set_pixel(x: i32, y: i32, color_index: i32, palette_index: i32);
    pub fn height() -> i32;
    pub fn width() -> i32;
    pub fn circle(x: i32, y: i32, radius: i32, color_index: i32, palette_index: i32);
    pub fn rect(x: i32, y: i32, width: i32, height: i32, color_index: i32, palette_index: i32);
    pub fn rect_filled(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        color_index: i32,
        palette_index: i32,
    );
    pub fn line(x0: i32, y0: i32, x1: i32, y1: i32, color_index: i32, palette_index: i32);
    pub fn sprite(sheet_index: i32, sprite_index: i32, x: i32, y: i32, palette_index: i32);
}

// Input
extern "C" {
    pub fn button_a_pressed(player_id: i32) -> i32;
    pub fn button_a_released(player_id: i32) -> i32;
    pub fn button_a_held(player_id: i32) -> i32;
    pub fn button_b_pressed(player_id: i32) -> i32;
    pub fn button_b_released(player_id: i32) -> i32;
    pub fn button_b_held(player_id: i32) -> i32;
    pub fn button_c_pressed(player_id: i32) -> i32;
    pub fn button_c_released(player_id: i32) -> i32;
    pub fn button_c_held(player_id: i32) -> i32;
    pub fn button_d_pressed(player_id: i32) -> i32;
    pub fn button_d_released(player_id: i32) -> i32;
    pub fn button_d_held(player_id: i32) -> i32;
    pub fn button_up_pressed(player_id: i32) -> i32;
    pub fn button_up_released(player_id: i32) -> i32;
    pub fn button_up_held(player_id: i32) -> i32;
    pub fn button_down_pressed(player_id: i32) -> i32;
    pub fn button_down_released(player_id: i32) -> i32;
    pub fn button_down_held(player_id: i32) -> i32;
    pub fn button_left_pressed(player_id: i32) -> i32;
    pub fn button_left_released(player_id: i32) -> i32;
    pub fn button_left_held(player_id: i32) -> i32;
    pub fn button_right_pressed(player_id: i32) -> i32;
    pub fn button_right_released(player_id: i32) -> i32;
    pub fn button_right_held(player_id: i32) -> i32;
    pub fn button_start_pressed(player_id: i32) -> i32;
    pub fn button_start_released(player_id: i32) -> i32;
    pub fn button_start_held(player_id: i32) -> i32;
    pub fn button_select_pressed(player_id: i32) -> i32;
    pub fn button_select_released(player_id: i32) -> i32;
    pub fn button_select_held(player_id: i32) -> i32;
    pub fn button_left_shoulder_pressed(player_id: i32) -> i32;
    pub fn button_left_shoulder_released(player_id: i32) -> i32;
    pub fn button_left_shoulder_held(player_id: i32) -> i32;
    pub fn button_right_shoulder_pressed(player_id: i32) -> i32;
    pub fn button_right_shoulder_released(player_id: i32) -> i32;
    pub fn button_right_shoulder_held(player_id: i32) -> i32;
    pub fn button_left_stick_pressed(player_id: i32) -> i32;
    pub fn button_left_stick_released(player_id: i32) -> i32;
    pub fn button_left_stick_held(player_id: i32) -> i32;
    pub fn button_right_stick_pressed(player_id: i32) -> i32;
    pub fn button_right_stick_released(player_id: i32) -> i32;
    pub fn button_right_stick_held(player_id: i32) -> i32;
    pub fn button_left_trigger_pressed(player_id: i32) -> i32;
    pub fn button_left_trigger_released(player_id: i32) -> i32;
    pub fn button_left_trigger_held(player_id: i32) -> i32;
    pub fn button_right_trigger_pressed(player_id: i32) -> i32;
    pub fn button_right_trigger_released(player_id: i32) -> i32;
    pub fn button_right_trigger_held(player_id: i32) -> i32;
    pub fn analog_left_x(player_id: i32) -> f32;
    pub fn analog_left_y(player_id: i32) -> f32;
    pub fn analog_right_x(player_id: i32) -> f32;
    pub fn analog_right_y(player_id: i32) -> f32;
    pub fn trigger_left(player_id: i32) -> f32;
    pub fn trigger_right(player_id: i32) -> f32;
}
