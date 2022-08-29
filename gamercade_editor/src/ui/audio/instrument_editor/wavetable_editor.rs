use eframe::{
    egui::{
        plot::{HLine, Line, Plot, PlotPoint, PlotPoints, VLine},
        Ui,
    },
    epaint::{Color32, Vec2},
};
use gamercade_audio::{WavetableBitDepth, WavetableDefinition};

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
        let last_index = instrument.data.len() - 1;

        // Draw the waveform:
        // Should we use a shape here instead of a plot?
        let points: PlotPoints = instrument
            .data
            .iter()
            .enumerate()
            .map(|(index, val)| [index as f64, *val as f64])
            .collect();
        let line = Line::new(points).color(Color32::GREEN);

        let lerp_segment = vec![
            [last_index as f64, instrument.data[last_index] as f64],
            [(last_index + 1) as f64, instrument.data[0] as f64],
        ];
        let line_segment = Line::new(lerp_segment).color(Color32::DARK_GREEN);

        let primary_pointer_down = ui.input().pointer.primary_down();

        Plot::new("my_plot")
            .width(1000.0)
            .height(200.0)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .allow_zoom(false)
            .set_margin_fraction(Vec2::ZERO)
            .include_x(-1.0)
            .include_x(last_index as f64 + 2.0)
            .include_y(WavetableBitDepth::MAX as f64 * 1.1)
            .include_y(WavetableBitDepth::MIN as f64 * 1.1)
            .label_formatter(move |_, point| {
                let (x, y) = plot_point_to_x_y(point, last_index);
                format!("Idx:{}\nVal:{}", x, y)
            })
            .show(ui, |plot_ui| {
                plot_ui.line(line);
                plot_ui.line(line_segment);

                plot_ui.hline(HLine::new(WavetableBitDepth::MAX as f64).color(Color32::RED));
                plot_ui.hline(HLine::new(WavetableBitDepth::MIN as f64).color(Color32::RED));
                plot_ui.vline(VLine::new(0.0).color(Color32::RED));
                plot_ui.vline(VLine::new(last_index as f64).color(Color32::RED));

                if plot_ui.plot_hovered() && primary_pointer_down {
                    let point = plot_ui.pointer_coordinate().unwrap();
                    let (x, y) = plot_point_to_x_y(&point, last_index);

                    // Only update if we changed the value!
                    if instrument.data[x] != y {
                        sync.notify_rom_changed();
                        instrument.data[x] = y;
                    }
                }
            });

        // TODO: Add something to add/shorten the length
        // TODO: Add wavetable generator helper UI

        EnvelopeWidget::draw(ui, &mut instrument.envelope, sync)
    }
}

fn plot_point_to_x_y(point: &PlotPoint, last_index: usize) -> (usize, WavetableBitDepth) {
    let x = (point.x.round() as usize).min(last_index);
    let y = point
        .y
        .round()
        .clamp(WavetableBitDepth::MIN as f64, WavetableBitDepth::MAX as f64)
        as WavetableBitDepth;

    (x, y)
}
