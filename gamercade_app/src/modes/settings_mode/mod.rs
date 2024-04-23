use crate::app::AppDrawContext;

#[derive(Default)]
pub struct SettingsModeView {}

impl SettingsModeView {
    pub fn draw(&mut self, context: &mut AppDrawContext) {
        let AppDrawContext { ui, .. } = context;
        ui.label("Settings Mode View");
    }

    // TODO: This func
    fn draw_controls_settings(&mut self, context: &mut AppDrawContext) {
        // Need to move console's input definitions to a publc crate like core
    }
}
