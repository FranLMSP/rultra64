use eframe::{egui, epi};
use std::cell::RefCell;
use std::rc::Rc;

use crate::emulator::Emulator;

#[derive(PartialEq, Eq)]
enum Register {
    CPU,
    CP0,
}

impl Default for Register {
    fn default() -> Self {
        Self::CPU
    }
}

pub struct EmulatorApp {
    core: Emulator,
    selected_register: Register,
}

impl Default for EmulatorApp {
    fn default() -> Self {
        Self {
            core: Emulator::new_hle(),
            selected_register: Register::CPU,
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
        let Self { core: emulator_core, selected_register } = self;

        let emulator_core = Rc::new(RefCell::new(emulator_core));
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Load ROM").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            let picked_path = path.display().to_string();
                            if let Ok(rom) = crate::rom::ROM::new_from_filename(&picked_path) {
                                let mut emulator_core = emulator_core.borrow_mut();
                                emulator_core.reload_hle();
                                emulator_core.mut_mmu().set_rom(rom);
                                emulator_core.mut_mmu().hle_ipl();
                                println!("ROM loaded!");
                            }
                        }
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        build_registers_window(ctx, selected_register, emulator_core.clone());
        build_emulator_controls_window(ctx, emulator_core.clone());
    }
}

fn build_registers_window(ctx: &egui::CtxRef, selected_register: &mut Register, emulator_core: Rc<RefCell<&mut Emulator>>) {
    egui::Window::new("Registers").vscroll(true).show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(selected_register, Register::CPU, "CPU");
            ui.selectable_value(selected_register, Register::CP0, "CP0");
        });
        ui.separator();
        match selected_register {
            Register::CPU => build_cpu_registers(ui, emulator_core),
            Register::CP0 => {ui.label("CP0 registers");},
        };
    });
}

fn build_cpu_registers(ui: &mut egui::Ui, emulator_core: Rc<RefCell<&mut Emulator>>) {
    let emulator_core = emulator_core.borrow();
    ui.columns(3, |cols| {
        cols[0].label("#");
        cols[1].label("Name");
        cols[2].label("Value");
    });
    ui.separator();
    ui.columns(3, |cols| {
        cols[0].label("-");
        cols[1].label("PC");
        cols[2].label(format!("{:64X}", emulator_core.cpu().registers().get_program_counter()));
    });
    ui.columns(3, |cols| {
        cols[0].label("-");
        cols[1].label("hi");
        cols[2].label(format!("{}", emulator_core.cpu().registers().get_hi()));
    });
    ui.columns(3, |cols| {
        cols[0].label("-");
        cols[1].label("lo");
        cols[2].label(format!("{}", emulator_core.cpu().registers().get_lo()));
    });
    for (index, name) in crate::registers::CPU_REGISTER_NAMES.into_iter().enumerate() {
        let val = emulator_core.cpu().registers().get_by_name(name);
        ui.columns(3, |cols| {
            cols[0].label(format!("r{}", index));
            cols[1].label(format!("{}", name));
            cols[2].label(format!("{}", val));
        });
    }
}

fn build_emulator_controls_window(ctx: &egui::CtxRef, emulator_core: Rc<RefCell<&mut Emulator>>) {
    egui::Window::new("Controls").vscroll(true).show(ctx, |ui| {
        if ui.button("Tick").clicked() {
            emulator_core.borrow_mut().tick();
        }
    });
}