use bytemuck::{Pod, Zeroable};

#[derive(Debug, Default, Clone, Copy, Pod, Zeroable, PartialEq, Eq)]
#[repr(C)]
pub struct MouseState(pub u32);

// Uses 25/32 bits
const MASK: u32 = 0b111_1111_1111; // 11 bits
const X_SHIFT: u32 = 0;
const Y_SHIFT: u32 = 11;
const LEFT_BUTTON_SHIFT: u32 = 22;
const RIGHT_BUTTON_SHIFT: u32 = 23;
const MIDDLE_BUTTON_SHIFT: u32 = 24;

impl MouseState {
    pub fn get_x_pos(self) -> u32 {
        (self.0 & MASK << X_SHIFT) >> X_SHIFT
    }

    pub fn get_y_pos(self) -> u32 {
        (self.0 & MASK << Y_SHIFT) >> Y_SHIFT
    }

    pub fn get_left_button_down(self) -> bool {
        self.0 & 1 << LEFT_BUTTON_SHIFT != 0
    }

    pub fn get_right_button_down(self) -> bool {
        self.0 & 1 << RIGHT_BUTTON_SHIFT != 0
    }

    pub fn get_middle_button_down(self) -> bool {
        self.0 & 1 << MIDDLE_BUTTON_SHIFT != 0
    }

    pub fn set_x_pos(&mut self, value: u32) {
        self.0 &= !(MASK << X_SHIFT);
        self.0 |= value << X_SHIFT;
    }

    pub fn set_y_pos(&mut self, value: u32) {
        self.0 &= !(MASK << Y_SHIFT);
        self.0 |= value << Y_SHIFT;
    }

    pub fn set_left_button(&mut self, value: bool) {
        let value = value as u32;
        self.0 &= !(value << LEFT_BUTTON_SHIFT);
        self.0 |= value << LEFT_BUTTON_SHIFT;
    }

    pub fn set_middle_button(&mut self, value: bool) {
        let value = value as u32;
        self.0 &= !(value << MIDDLE_BUTTON_SHIFT);
        self.0 |= value << MIDDLE_BUTTON_SHIFT;
    }

    pub fn set_right_button(&mut self, value: bool) {
        let value = value as u32;
        self.0 &= !(value << RIGHT_BUTTON_SHIFT);
        self.0 |= value << RIGHT_BUTTON_SHIFT;
    }
}

#[cfg(test)]
mod tests {
    use crate::MouseState;

    #[test]
    fn test_mouse_state() {
        let mut out = MouseState::default();
        let x = 1920 - 1;
        let y = 1080 - 1;
        out.set_x_pos(x);
        out.set_y_pos(y);

        assert_eq!(out.get_x_pos(), x);
        assert_eq!(out.get_y_pos(), y);

        assert_eq!(out.get_left_button_down(), false);
        assert_eq!(out.get_middle_button_down(), false);
        assert_eq!(out.get_right_button_down(), false);

        out.set_x_pos(0);
        out.set_y_pos(0);
        assert_eq!(out.get_x_pos(), 0);
        assert_eq!(out.get_y_pos(), 0);
        assert_eq!(out.0, 0);

        out.set_left_button(false);
        out.set_middle_button(false);
        out.set_right_button(false);
        assert_eq!(out.0, 0);

        out.set_left_button(true);
        out.set_middle_button(true);
        out.set_right_button(true);
        assert_eq!(out.get_left_button_down(), true);
        assert_eq!(out.get_middle_button_down(), true);
        assert_eq!(out.get_right_button_down(), true);

        out.set_x_pos(123);
        assert_eq!(out.get_x_pos(), 123);
    }
}
