use egui::Widget;

use crate::frame_history::FrameHistory;

/// How often we repaint the demo app by default
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunMode
{
    Reactive,
    Continuous,
}

impl Default for RunMode
{
    fn default() -> Self
    {
        Self::Reactive
    }
}

// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(default))]
pub struct BackendPanel
{
    pub open: bool,

    // #[cfg_attr(feature = "serde", serde(skip))]
    // go back to [`RunMode::Reactive`] mode each time we start
    run_mode: RunMode,

    // #[cfg_attr(feature = "serde", serde(skip))]
    repaint_after_seconds: f32,

    /// current slider value for current gui scale
    // #[cfg_attr(feature = "serde", serde(skip))]
    pixels_per_point: Option<f32>,

    // #[cfg_attr(feature = "serde", serde(skip))]
    frame_history: crate::frame_history::FrameHistory,

    egui_windows: EguiWindows,
}

impl Default for BackendPanel
{
    fn default() -> Self
    {
        Self {
            open: false,
            run_mode: RunMode::default(),
            repaint_after_seconds: 1.0,
            pixels_per_point: None,
            frame_history: FrameHistory::default(),
            egui_windows: EguiWindows::default(),
        }
    }
}

impl BackendPanel
{
    pub fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame)
    {
        self.frame_history
            .on_new_frame(ctx.input().time, frame.info().cpu_usage);

        match self.run_mode {
            RunMode::Continuous => {
                // Tell the backend to repaint as soon as possible
                ctx.request_repaint();
            }
            RunMode::Reactive => {
                // let the computer rest for a bit
                ctx.request_repaint_after(std::time::Duration::from_secs_f32(
                    self.repaint_after_seconds,
                ));
            }
        }
    }

    pub fn end_of_frame(&mut self, ctx: &egui::Context)
    {
        self.egui_windows.windows(ctx);
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame)
    {
        egui::trace!(ui);

        self.integration_ui(ui, frame);

        ui.separator();

        self.run_mode_ui(ui);

        ui.separator();

        self.frame_history.ui(ui);

        ui.separator();

        ui.label("egui windows:");
        self.egui_windows.checkboxes(ui);

        ui.separator();

        #[cfg(not(target_arch = "wasm32"))]
        {
            ui.separator();
            if ui.button("Quit").clicked() {
                frame.close();
            }
        }
    }

    fn integration_ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame)
    {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("Course Project: ");
            ui.hyperlink_to(
                "GitHub",
                "https://github.com/archeoss/bmstu-cg-courseProject-5th-sem",
            );
            ui.label(".");
        });

        #[cfg(target_arch = "wasm32")]
        ui.collapsing("Web info (location)", |ui| {
            ui.monospace(format!("{:#?}", frame.info().web_info.location));
        });

        // On web, the browser controls `pixels_per_point`.
        let integration_controls_pixels_per_point = frame.is_web();
        if !integration_controls_pixels_per_point {
            self.pixels_per_point_ui(ui, &frame.info());
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            ui.horizontal(|ui| {
                let mut fullscreen = frame.info().window_info.fullscreen;
                if ui
                    .checkbox(&mut fullscreen, "🗖 Fullscreen (F11)")
                    .on_hover_text("Fullscreen the window")
                    .changed()
                {
                    frame.set_fullscreen(fullscreen);
                }
            });
        }
    }

    fn pixels_per_point_ui(&mut self, ui: &mut egui::Ui, info: &eframe::IntegrationInfo)
    {
        let pixels_per_point = self
            .pixels_per_point
            .get_or_insert_with(|| ui.ctx().pixels_per_point());

        let mut reset = false;

        ui.horizontal(|ui| {
            ui.spacing_mut().slider_width = 90.0;

            let response = ui
                .add(
                    egui::Slider::new(pixels_per_point, 0.5..=5.0)
                        .logarithmic(true)
                        .clamp_to_range(true)
                        .text("Scale"),
                )
                .on_hover_text("Physical pixels per point.");

            if response.drag_released() {
                // We wait until mouse release to activate:
                ui.ctx().set_pixels_per_point(*pixels_per_point);
                reset = true;
            } else if !response.is_pointer_button_down_on() {
                // When not dragging, show the current pixels_per_point so others can change it.
                reset = true;
            }

            if let Some(native_pixels_per_point) = info.native_pixels_per_point {
                let enabled = ui.ctx().pixels_per_point() != native_pixels_per_point;
                if ui
                    .add_enabled(enabled, egui::Button::new("Reset"))
                    .on_hover_text(format!(
                        "Reset scale to native value ({native_pixels_per_point:.1})"
                    ))
                    .clicked()
                {
                    ui.ctx().set_pixels_per_point(native_pixels_per_point);
                }
            }
        });

        if reset {
            self.pixels_per_point = None;
        }
    }

    fn run_mode_ui(&mut self, ui: &mut egui::Ui)
    {
        ui.horizontal(|ui| {
            let run_mode = &mut self.run_mode;
            ui.label("Mode:");
            ui.radio_value(run_mode, RunMode::Reactive, "Reactive")
                .on_hover_text("Repaint when there are animations or input (e.g. mouse movement)");
            ui.radio_value(run_mode, RunMode::Continuous, "Continuous")
                .on_hover_text("Repaint everything each frame");
        });

        if self.run_mode == RunMode::Continuous {
            ui.label(format!(
                "Repainting the UI each frame. FPS: {:.1}",
                self.frame_history.fps()
            ));
        } else {
            ui.label("Only running UI code when there are animations or input.");

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("(but at least every ");
                egui::DragValue::new(&mut self.repaint_after_seconds)
                    .clamp_range(0.1..=10.0)
                    .speed(0.1)
                    .suffix(" s")
                    .ui(ui)
                    .on_hover_text("Repaint this often, even if there is no input.");
                ui.label(")");
            });
        }
    }
}

// ----------------------------------------------------------------------------
//
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct EguiWindows
{
    // egui stuff:
    settings: bool,
    inspection: bool,
    memory: bool,
    output_events: bool,

    // #[cfg_attr(feature = "serde", serde(skip))]
    output_event_history: std::collections::VecDeque<egui::output::OutputEvent>,
}

impl Default for EguiWindows
{
    fn default() -> Self
    {
        Self::none()
    }
}

impl EguiWindows
{
    fn none() -> Self
    {
        Self {
            settings: false,
            inspection: false,
            memory: false,
            output_events: false,
            output_event_history: std::collections::VecDeque::default(),
        }
    }

    fn checkboxes(&mut self, ui: &mut egui::Ui)
    {
        let Self {
            settings,
            inspection,
            memory,
            output_events,
            output_event_history: _,
        } = self;

        ui.checkbox(settings, "🔧 Settings");
        ui.checkbox(inspection, "🔍 Inspection");
        ui.checkbox(memory, "📝 Memory");
        ui.checkbox(output_events, "📤 Output Events");
    }

    fn windows(&mut self, ctx: &egui::Context)
    {
        let Self {
            settings,
            inspection,
            memory,
            output_events,
            output_event_history,
        } = self;

        for event in &ctx.output().events {
            output_event_history.push_back(event.clone());
        }
        while output_event_history.len() > 1000 {
            output_event_history.pop_front();
        }

        egui::Window::new("🔧 Settings")
            .open(settings)
            .vscroll(true)
            .show(ctx, |ui| {
                ctx.settings_ui(ui);
            });

        egui::Window::new("🔍 Inspection")
            .open(inspection)
            .vscroll(true)
            .show(ctx, |ui| {
                ctx.inspection_ui(ui);
            });

        egui::Window::new("📝 Memory")
            .open(memory)
            .resizable(false)
            .show(ctx, |ui| {
                ctx.memory_ui(ui);
            });

        egui::Window::new("📤 Output Events")
            .open(output_events)
            .resizable(true)
            .default_width(520.0)
            .show(ctx, |ui| {
                ui.label(
                    "Recent output events from egui. \
            These are emitted when you interact with widgets, or move focus between them with TAB. \
            They can be hooked up to a screen reader on supported platforms.",
                );

                ui.separator();

                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for event in output_event_history {
                            ui.label(format!("{event:?}"));
                        }
                    });
            });
    }
}
