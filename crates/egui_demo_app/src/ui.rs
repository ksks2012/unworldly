use egui::{Context, ScrollArea};

/// A menu bar in which you can select different demo windows to show.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DemoWindows {
    about_is_open: bool,
    // about: About,
    // demos: Demos,
    // tests: Tests,
}

impl Default for DemoWindows {
    fn default() -> Self {
        Self {
            about_is_open: true,
            // about: Default::default(),
            // demos: Default::default(),
            // tests: Default::default(),
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
                    ui.heading("✒ egui demos");
                });

                ui.separator();

                use egui::special_emojis::{GITHUB, TWITTER};
                ui.hyperlink_to(
                    format!("{GITHUB} egui on GitHub"),
                    "https://github.com/emilk/egui",
                );
                ui.hyperlink_to(
                    format!("{TWITTER} @ernerfeldt"),
                    "https://twitter.com/ernerfeldt",
                );

                ui.separator();

                self.demo_list_ui(ui);
            });

        // egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        //     egui::menu::bar(ui, |ui| {
        //     });
        // });

    }

    fn demo_list_ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                if ui.button("Organize windows").clicked() {
                    ui.ctx().memory_mut(|mem| mem.reset_areas());
                }
            });
        });
    }
}
