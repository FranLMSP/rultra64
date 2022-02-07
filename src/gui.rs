use eframe::{egui, epi};

use crate::emulator::Emulator;

pub struct EmulatorApp {
    core: Emulator,
}

impl Default for EmulatorApp {
    fn default() -> Self {
        Self {
            core: Emulator::new(),
        }
    }
}

impl epi::App for EmulatorApp {
    fn name(&self) -> &str {
        "Rultra64"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        let Self { core: emulator_core } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load ROM").clicked() {
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        build_cpu_registers_window(ctx, &emulator_core);
    }
}

fn build_cpu_registers_window(ctx: &egui::CtxRef, emulator_core: &Emulator) {
    egui::Window::new("CPU Registers").vscroll(true).show(ctx, |ui| {

        egui::SidePanel::left("left_panel")
            // .resizable(true)
            .default_width(10.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("# | Name");
                });
                for (index, name) in crate::registers::CPU_REGISTER_NAMES.into_iter().enumerate() {
                    ui.label(format!("r{} | {}", index, name));
                }
            });
        egui::SidePanel::right("right_panel")
            .resizable(true)
            // .default_width(50.0)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Value");
                });
                for (_, name) in crate::registers::CPU_REGISTER_NAMES.into_iter().enumerate() {
                    let val = emulator_core.cpu().registers().get_by_name(name);
                    ui.label(format!("{}", val));
                }
            });
    });
}