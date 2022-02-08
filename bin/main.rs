use rultra64::gui::EmulatorApp;

fn main() {
    let app = EmulatorApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}