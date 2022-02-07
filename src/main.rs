#![forbid(unsafe_code)]

#![warn(clippy::all, rust_2018_idioms)]

use std::thread::sleep;
use egui_test::RT;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = egui_test::TemplateApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
