#![windows_subsystem = "windows"]

mod app;
mod language;
mod file_manager;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 650.0])
            .with_min_inner_size([700.0, 500.0])
            .with_title("SFS Language Tool"),
        ..Default::default()
    };

    eframe::run_native(
        "SFS Language Tool",
        options,
        Box::new(|cc| {
            let ctx = &cc.egui_ctx;
            
            ctx.style_mut(|style| {
                style.visuals = egui::Visuals::dark();
            });
            
            let mut fonts = egui::FontDefinitions::default();

            fonts.font_data.insert("cn".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/NotoSansSC.ttf")));

            fonts.font_data.insert("arial".to_owned(),
                egui::FontData::from_static(include_bytes!("../assets/fonts/arial.ttf")));
            
            if let Ok(data) = std::fs::read("C:/Windows/Fonts/malgun.ttf") {
                fonts.font_data.insert("malgun".to_owned(), 
                    egui::FontData::from_owned(data));
            }
            
            if let Ok(data) = std::fs::read("C:/Windows/Fonts/Nirmala.ttf") {
                fonts.font_data.insert("nirmala".to_owned(), 
                    egui::FontData::from_owned(data));
            }
            
            if let Ok(data) = std::fs::read("C:/Windows/Fonts/seguiemj.ttf") {
                fonts.font_data.insert("segoe".to_owned(), 
                    egui::FontData::from_owned(data));
            }
            
            fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
                .splice(0..0, vec!["cn".to_owned(), "arial".to_owned(), "malgun".to_owned(), "nirmala".to_owned(), "segoe".to_owned()]);
            fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
                .splice(0..0, vec!["cn".to_owned(), "arial".to_owned(), "malgun".to_owned(), "nirmala".to_owned(), "segoe".to_owned()]);
            
            ctx.set_fonts(fonts);
            
            Box::new(app::SfsLanguageApp::new())
        }),
    )
}
