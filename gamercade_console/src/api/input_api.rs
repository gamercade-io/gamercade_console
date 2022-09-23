use paste::paste;

macro_rules! derive_bind_input_api {
    (
        Buttons { $($btn_name:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
        Mouse {
            Buttons { $($mbtn_name:ident,)* },
            Axis { $($maxis_name:ident,)* },
            Wheel { $($mwheel_name:ident,)* },
         },
    ) => {
        paste! {
            pub trait InputApi {
                $(
                    fn [<button_ $btn_name _pressed>](&self, player_id: i32) -> i32;
                    fn [<button_ $btn_name _released>](&self, player_id: i32) -> i32;
                    fn [<button_ $btn_name _held>](&self, player_id: i32) -> i32;
                )*

                $(
                    fn [<analog_ $anlg_name _x>](&self, player_id: i32) -> f32;
                    fn [<analog_ $anlg_name _y>](&self, player_id: i32) -> f32;
                )*

                $(
                    fn [<trigger_ $trg_name>](&self, player_id: i32) -> f32;
                )*

                $(
                    fn [<mouse_ $mbtn_name _pressed>](&self, player_id: i32) -> i32;
                    fn [<mouse_ $mbtn_name _released>](&self, player_id: i32) -> i32;
                    fn [<mouse_ $mbtn_name _held>](&self, player_id: i32) -> i32;
                )*

                $(
                    fn [<mouse_ $maxis_name _pos>](&self, player_id: i32) -> i32;
                    fn [<mouse_ $maxis_name _delta>](&self, player_id: i32) -> i32;
                )*

                $(
                    fn [<mouse_wheel_ $mwheel_name>](&self, player_id: i32) -> i32;
                )*

                fn raw_input_state(&self, player_id: i32) -> i64;
                fn raw_mouse_state(&self, player_id: i32) -> i64;
            }

            pub trait InputApiBinding {
                $(
                    fn [<bind_button_ $btn_name _pressed>](&mut self);
                    fn [<bind_button_ $btn_name _released>](&mut self);
                    fn [<bind_button_ $btn_name _held>](&mut self);
                )*

                $(
                    fn [<bind_analog_ $anlg_name _x>](&mut self);
                    fn [<bind_analog_ $anlg_name _y>](&mut self);
                )*

                $(
                    fn [<bind_trigger_ $trg_name>](&mut self);
                )*

                $(
                    fn [<bind_mouse_ $mbtn_name _pressed>](&mut self);
                    fn [<bind_mouse_ $mbtn_name _released>](&mut self);
                    fn [<bind_mouse_ $mbtn_name _held>](&mut self);
                )*

                $(
                    fn [<bind_mouse_ $maxis_name _pos>](&mut self);
                    fn [<bind_mouse_ $maxis_name _delta>](&mut self);
                )*

                $(
                    fn [<bind_mouse_wheel_ $mwheel_name>](&mut self);
                )*

                fn bind_raw_input_state(&mut self);
                fn bind_raw_mouse_state(&mut self);

                fn bind_input_api(&mut self) {
                    $(
                        self.[<bind_button_ $btn_name _pressed>]();
                        self.[<bind_button_ $btn_name _released>]();
                        self.[<bind_button_ $btn_name _held>]();
                    )*

                    $(
                        self.[<bind_analog_ $anlg_name _x>]();
                        self.[<bind_analog_ $anlg_name _y>]();
                    )*

                    $(
                        self.[<bind_trigger_ $trg_name>]();
                    )*

                    $(
                        self.[<bind_mouse_ $mbtn_name _pressed>]();
                        self.[<bind_mouse_ $mbtn_name _released>]();
                        self.[<bind_mouse_ $mbtn_name _held>]();
                    )*

                    $(
                        self.[<bind_mouse_ $maxis_name _pos>]();
                        self.[<bind_mouse_ $maxis_name _delta>]();
                    )*

                    $(
                        self.[<bind_mouse_wheel_ $mwheel_name>]();
                    )*

                    self.bind_raw_input_state();
                    self.bind_raw_mouse_state();
                }
            }
        }
    };
}

derive_bind_input_api! {
    Buttons {
        a,
        b,
        c,
        d,
        up,
        down,
        left,
        right,
        start,
        select,
        left_shoulder,
        right_shoulder,
        left_stick,
        right_stick,
        left_trigger,
        right_trigger,
    },
    Analogs {
        left,
        right,
    },
    Triggers {
        left,
        right,
    },
    Mouse {
        Buttons {
            left,
            right,
            middle,
        },
        Axis {
            x,
            y,
        },
        Wheel {
            up,
            down,
            left,
            right,
        },
    },
}
