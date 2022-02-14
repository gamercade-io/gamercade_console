use paste::paste;

macro_rules! derive_bind_input_api {
    (
        Buttons { $($btn_name:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
    ) => {
        paste! {
            pub trait InputApi {
                $(
                    fn [<button_ $btn_name _pressed>](&self, player_id: u8) -> bool;
                    fn [<button_ $btn_name _released>](&self, player_id: u8) -> bool;
                    fn [<button_ $btn_name _held>](&self, player_id: u8) -> bool;
                )*

                $(
                    fn [<analog_ $anlg_name _x>](&self, player_id: u8) -> f32;
                    fn [<analog_ $anlg_name _y>](&self, player_id: u8) -> f32;
                )*

                $(
                    fn [<trigger_ $trg_name>](&self, player_id: u8) -> f32;
                )*
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
}
