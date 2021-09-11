mod app;
mod settings;
use app::App;
use eframe::NativeOptions;

fn main() {
    eframe::run_native(Box::new(App::default()), NativeOptions::default());
}