use crate::app::AppDrawContext;

#[derive(Default)]
pub struct ArcadeModeView {}

impl ArcadeModeView {
    pub fn draw(&mut self, context: AppDrawContext) {
        let AppDrawContext { ui, .. } = context;
        ui.label("Arcade Mode");
    }
}
