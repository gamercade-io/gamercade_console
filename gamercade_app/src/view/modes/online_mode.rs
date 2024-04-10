use crate::app::AppDrawContext;

#[derive(Default)]
pub struct OnlineModeView {}

impl OnlineModeView {
    pub fn draw(&mut self, context: AppDrawContext) {
        let AppDrawContext { ui, .. } = context;
        ui.label("Online Mode");
    }
}
