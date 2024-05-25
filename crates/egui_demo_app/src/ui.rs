use std::collections::BTreeSet;

use egui::{Context, ScrollArea};

use crate::demo::About;
use crate::demo::Demo;

// ----------------------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
struct Demos {
    #[cfg_attr(feature = "serde", serde(skip))]
    demos: Vec<Box<dyn Demo>>,

    open: BTreeSet<String>,
}

impl Default for Demos {
    fn default() -> Self {
        Self::from_demos(vec![
            Box::<super::MiscDemoWindow>::default(),
        ])
    }
}

impl Demos {
    pub fn from_demos(demos: Vec<Box<dyn Demo>>) -> Self {
        let mut open = BTreeSet::new();

        Self { demos, open }
    }

    pub fn checkboxes(&mut self, ui: &mut egui::Ui) {
        let Self { demos, open } = self;
        for demo in demos {
            if demo.is_enabled(ui.ctx()) {
                let mut is_open = open.contains(demo.name());
                ui.toggle_value(&mut is_open, demo.name());
                set_open(open, demo.name(), is_open);
            }
        }
    }

    pub fn windows(&mut self, ctx: &Context) {
        let Self { demos, open } = self;
        for demo in demos {
            let mut is_open = open.contains(demo.name());
            demo.show(ctx, &mut is_open);
            set_open(open, demo.name(), is_open);
        }
    }
}

// ----------------------------------------------------------------------------

fn set_open(open: &mut BTreeSet<String>, key: &'static str, is_open: bool) {
    if is_open {
        if !open.contains(key) {
            open.insert(key.to_owned());
        }
    } else {
        open.remove(key);
    }
}

// ----------------------------------------------------------------------------

/// A menu bar in which you can select different demo windows to show.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DemoWindows {
    about_is_open: bool,
    about: About,
    demos: Demos,
}

impl Default for DemoWindows {
    fn default() -> Self {
        Self {
            about_is_open: true,
            about: Default::default(),
            demos: Default::default(),
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
        self.demos.windows(ctx);
    }

    fn demo_list_ui(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                ui.toggle_value(&mut self.about_is_open, self.about.name());

                ui.separator();
                self.demos.checkboxes(ui);
                ui.separator();
            });

        });
    }
}