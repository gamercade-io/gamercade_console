use eframe::egui::{
    plot::{Line, Plot, PlotPoints},
    Ui,
};
use gamercade_audio::WavetableDefinition;

use crate::ui::AudioSyncHelper;

use super::envelope_widget::EnvelopeWidget;

#[derive(Clone, Debug, Default)]
pub struct WavetableEditor {}

impl WavetableEditor {
    pub(crate) fn draw(
        &mut self,
        ui: &mut Ui,
        instrument: &mut WavetableDefinition,
        sync: &mut AudioSyncHelper,
    ) {
        // TODO: Write this
        ui.label("Wavetable Editor: TODO");

        // Draw the waveform:
        // Should we use a shape here instead of a plot?
        let points: PlotPoints = instrument
            .data
            .iter()
            .enumerate()
            .map(|(index, val)| [index as f64, *val as f64])
            .collect();
        let line = Line::new(points);
        Plot::new("my_plot")
            .view_aspect(5.0)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .allow_zoom(false)
            .center_y_axis(true)
            .include_x(0.0)
            .include_x(instrument.data.len() as f64)
            .show(ui, |plot_ui| plot_ui.line(line));

        EnvelopeWidget::draw(ui, &mut instrument.envelope, sync)
    }
}
