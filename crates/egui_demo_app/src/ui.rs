use egui::{Context, ScrollArea};

use super::About;
use crate::Demo;

/// A menu bar in which you can select different demo windows to show.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DemoWindows {
    about_is_open: bool,
    about: About,
    // demos: Demos,
    // tests: Tests,
}

impl Default for DemoWindows {
    fn default() -> Self {
        Self {
            about_is_open: true,
            about: Default::default(),
        }
    }
}

impl DemoWindows {
    /// Show the app ui (menu bar and windows).
    pub fn ui(&mut self, ctx: &Context) {
        self.desktop_ui(ctx);
    }

    fn desktop_ui(&mut self, ctx: &Context) {
        egui::SidePanel::right("egui_demo_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("✒ test demos");
                });

                ui.separator();

                use egui::special_emojis::{GITHUB, TWITTER};
                ui.hyperlink_to(
                    format!("{GITHUB} GitHub"),
                    "https://github.com/",
                );
                ui.hyperlink_to(
                    format!("{TWITTER} @twitter"),
                    "https://twitter.com/",
                );

                ui.separator();

                self.demo_list_ui(ui);
            });

        self.show_windows(ctx);
    }

    /// Show the open windows.
    fn show_windows(&mut self, ctx: &Context) {
        self.about.show(ctx, &mut self.about_is_open);
    }

    fn demo_list_ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                ui.toggle_value(&mut self.about_is_open, self.about.name());

                ui.separator();
            });

        });
    }
}