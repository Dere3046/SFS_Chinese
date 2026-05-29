#![windows_subsystem = "windows"]

mod app;
mod file_manager;

use eframe::egui;
use app::SfsLanguageApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 400.0])
            .with_min_inner_size([600.0, 350.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "SFS Language Tool",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            let font_data = egui::FontData::from_static(include_bytes!("../assets/NotoSansSC.ttf"));
            fonts.font_data.insert("noto_sc".to_owned(), font_data);
            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "noto_sc".to_owned());
            fonts
                .families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .insert(0, "noto_sc".to_owned());
            cc.egui_ctx.set_fonts(fonts);

            Box::new(SfsLanguageApp::new())
        }),
    )
}

fn load_icon() -> egui::IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(include_bytes!("../assets/app.ico"))
            .expect("Failed to load icon")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    egui::IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}