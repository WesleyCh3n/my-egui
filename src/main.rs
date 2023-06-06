#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

pub struct MyApp {}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        transparent: true,
        ..Default::default()
    };
    eframe::run_native(
        "My Egui App",
        native_options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

impl eframe::App for MyApp {
    fn update(
        &mut self,
        ctx: &eframe::egui::Context,
        frame: &mut eframe::Frame,
    ) {
        egui::TopBottomPanel::top("menu bar")
            .frame(egui::Frame::none()) // can fill(Color32)
            .show(ctx, |ui| self.top_panel(ui, frame));
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| self.center_panel(ui, frame));
    }
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let _ctx = _cc.egui_ctx.clone();
        // _ctx can send to different thread and _ctx.request_repaint()
        Self {}
    }
    fn top_panel(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("Menu", |ui| {
                if ui.button("Close").clicked() {
                    frame.close()
                }
            })
        });
    }
    fn center_panel(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.label("Hello World");
    }
}
