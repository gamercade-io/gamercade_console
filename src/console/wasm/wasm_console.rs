use std::sync::Arc;

use crate::{core::Rom, Console};

pub struct WasmConsole {
    rom: Arc<Rom>,
    frame_buffer: Box<[u8]>,
}

impl Console for WasmConsole {
    fn call_init(&self) {
        todo!()
    }

    fn call_update(&self) {
        todo!()
    }
    
    fn call_draw(&self) {
        todo!()
    }

    fn rom(&self) -> &Rom {
        &self.rom
    }

    fn blit(&self, buffer: &mut [u8]) {
        buffer.copy_from_slice(&self.frame_buffer);
    }

    fn handle_requests(&mut self, requests: Vec<ggrs::GGRSRequest<crate::GGRSConfig>>) {
        todo!()
    }
}
