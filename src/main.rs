mod app;
fn main() {
    let option = eframe::NativeOptions{ default_theme:eframe::Theme::Light, ..Default::default() };
    eframe::run_native(
        "calculator",
        option,
        Box::new(|_cc| Box::new(app::MyApp::new(_cc))),
    );
}
