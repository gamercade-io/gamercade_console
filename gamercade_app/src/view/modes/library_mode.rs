use crate::app::AppDrawContext;

#[derive(Default)]
pub struct LibraryModeView {}

impl LibraryModeView {
    pub fn draw(&mut self, context: AppDrawContext) {
        let AppDrawContext { ui, .. } = context;
        ui.label("Library Mode");
    }
}
