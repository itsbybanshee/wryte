//! Help › Check for Updates. Wryte never contacts an update server: the window says which version
//! is running and where new releases are published.

use crate::WordApp;

/// Where new versions are published.
pub const RELEASES_URL: &str = "https://github.com/itsbybanshee/wryte/releases";

#[derive(Default)]
pub(crate) struct Runtime {
    pub(crate) manual_window_open: bool,
    pub(crate) status_message: Option<String>,
}

pub(crate) fn check(app: &mut WordApp) {
    app.updates_rt.manual_window_open = true;
    app.updates_rt.status_message = Some(
        tl!("You're running Wryte {version}. Wryte doesn't check for updates on its own; new versions are published on GitHub.")
            .replace("{version}", env!("CARGO_PKG_VERSION")),
    );
}

pub(crate) fn tick(app: &mut WordApp, ctx: &egui::Context) {
    let _ = (app, ctx);
}

pub(crate) fn show(app: &mut WordApp, ctx: &egui::Context) {
    if !app.updates_rt.manual_window_open {
        return;
    }
    let mut open = true;
    let mut close = false;
    egui::Window::new(tl!("Check for Updates"))
        .open(&mut open)
        .resizable(false)
        .collapsible(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            if let Some(msg) = &app.updates_rt.status_message {
                ui.label(msg);
            }
            ui.hyperlink_to(tl!("Open the releases page"), RELEASES_URL);
            ui.add_space(8.0);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(tl!("Close")).clicked() {
                    close = true;
                }
            });
        });
    if !open || close {
        app.updates_rt.manual_window_open = false;
        app.updates_rt.status_message = None;
    }
}
