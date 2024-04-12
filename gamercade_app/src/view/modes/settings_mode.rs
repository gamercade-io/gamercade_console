use crate::app::AppDrawContext;

#[derive(Default)]
pub struct SettingsModeView {}

impl SettingsModeView {
    pub fn draw(&mut self, context: AppDrawContext) {
        let AppDrawContext { ui, .. } = context;
        ui.label("Settings View");
    }
}
