mod app;
mod disasm;
mod editor;
mod extensions;
mod project;
mod theme;
mod toolchain;
mod ui;
mod wizard;

use app::{AppScreen, FasdeqApp};
use eframe::egui;

struct RootApp {
    inner: FasdeqApp,
}

impl RootApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let inner = FasdeqApp::new();
        inner.apply_theme(&cc.egui_ctx);

        if inner.toolchain_config.setup_completed {
            let mut app = inner;
            app.extension_manager.load_all_enabled();
            Self { inner: app }
        } else {
            Self { inner }
        }
    }
}

impl eframe::App for RootApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.inner.screen {
            AppScreen::FirstRunWizard => ui::wizard_screen::draw(&mut self.inner, ctx),
            AppScreen::StartMenu => ui::start_menu::draw(&mut self.inner, ctx),
            AppScreen::NewProjectMenu => ui::new_project::draw(&mut self.inner, ctx),
            AppScreen::CloneProject => ui::clone_screen::draw(&mut self.inner, ctx),
            AppScreen::Workspace => ui::workspace_screen::draw(&mut self.inner, ctx),
        }
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([960.0, 600.0])
            .with_title("Fasdeq Studio")
            .with_icon(load_icon()),
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "Fasdeq Studio",
        native_options,
        Box::new(|cc| Ok(Box::new(RootApp::new(cc)))),
    )
}

fn load_icon() -> egui::IconData {
    let bytes = include_bytes!("../assets/icon/fasdeq-icon-256.png");
    match image::load_from_memory(bytes) {
        Ok(image) => {
            let rgba = image.to_rgba8();
            let (width, height) = rgba.dimensions();
            egui::IconData {
                rgba: rgba.into_raw(),
                width,
                height,
            }
        }
        Err(_) => egui::IconData {
            rgba: vec![0; 4],
            width: 1,
            height: 1,
        },
    }
}
