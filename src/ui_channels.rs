use eframe::egui;

use crate::App;

pub fn ui_channels(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::left("channels")
        .default_width(220.0)
        .min_width(180.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.add_space(8.0);
            ui.heading(app.selected_server().name.clone());
            ui.add_space(8.0);
            ui.separator();

            let channels = app.selected_server().channels.clone();
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (i, ch) in channels.iter().enumerate() {
                        let selected = i == app.selected_channel;
                        let label = format!("# {}", ch.name);
                        if ui
                            .add(egui::SelectableLabel::new(selected, label))
                            .clicked()
                        {
                            app.selected_channel = i;
                            app.scroll_to_bottom = true;
                        }
                    }
                });
        });
}
