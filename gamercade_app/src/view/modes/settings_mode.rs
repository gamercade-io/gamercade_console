use crate::app::AppDrawContext;

#[derive(Default)]
pub struct SettingsView {}

impl SettingsView {
    pub fn draw(&mut self, context: AppDrawContext) {
        let AppDrawContext { ui, .. } = context;
        ui.label("Settings View");
    }
}
