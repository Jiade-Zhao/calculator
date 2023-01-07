pub mod calculate;

// set the fonts
pub fn setup_custom_fonts(ctx: &eframe::egui::Context) {
    let mut fonts = eframe::egui::FontDefinitions::default();

    fonts.font_data.insert(
        "my_font".to_owned(),
        eframe::egui::FontData::from_static(include_bytes!("../consola.ttf")),
    );

    fonts
        .families
        .entry(eframe::egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    fonts
        .families
        .entry(eframe::egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    ctx.set_fonts(fonts);
    use eframe::egui::FontFamily::Proportional;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            eframe::egui::TextStyle::Heading,
            eframe::egui::FontId::new(30.0, Proportional),
        ),
        (eframe::egui::TextStyle::Body, eframe::egui::FontId::new(20.0, Proportional)),
        (
            eframe::egui::TextStyle::Monospace,
            eframe::egui::FontId::new(20.0, Proportional),
        ),
        (
            eframe::egui::TextStyle::Button,
            eframe::egui::FontId::new(30.0, Proportional),
        ),
        (eframe::egui::TextStyle::Small, eframe::egui::FontId::new(10.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
