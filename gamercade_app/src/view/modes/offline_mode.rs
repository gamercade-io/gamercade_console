use crate::app::AppDrawContext;

#[derive(Default)]
pub struct OfflineModeView {}

impl OfflineModeView {
    pub fn draw(&mut self, context: AppDrawContext) {
        let AppDrawContext { ui, .. } = context;
        ui.label("Offline Mode");
    }
}
