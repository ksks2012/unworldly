use egui::{Context, ScrollArea};

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct LogWindows {
    #[cfg_attr(feature = "serde", serde(skip))]
    output_event_history: std::collections::VecDeque<String>,
}

impl Default for LogWindows {
    fn default() -> Self {
        Self {
            output_event_history: Default::default(),
        }
    }
}

impl super::Demo for LogWindows {
    fn name(&self) -> &'static str {
        "📤 Output Events"
    }

    fn show(&mut self, ctx: &Context, open: &mut bool) {
        let Self {
            output_event_history,
        } = self;

        output_event_history.push_back(output_event_history.len().to_string());

        egui::Window::new("📤 Output Events")
            .open(open)
            .resizable(true)
            .default_width(520.0)
            .show(ctx, |ui| {
                ui.label(
                    "Testing log printing. This is a test of the emergency broadcast system. This is only a test",
                );

                ui.separator();

                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for event in output_event_history {
                            ui.label(format!("{event:?}"));
                        }
                    });
            });
    }
}