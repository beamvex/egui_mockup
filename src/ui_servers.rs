use eframe::egui;
use crate::App;

pub fn ui_servers(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::left("servers")
        .exact_width(64.0)
        .resizable(false)
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(0.0, 8.0);
            ui.add_space(8.0);

            for (i, server) in app.servers.iter().enumerate() {
                let selected = i == app.selected_server;
                let label = abbreviate(&server.name);
                let button = egui::SelectableLabel::new(selected, label);
                let resp = ui.add_sized([48.0, 48.0], button);
                if resp.clicked() {
                    app.selected_server = i;
                    app.selected_channel = 0;
                    app.scroll_to_bottom = true;
                }
                resp.on_hover_text(&server.name);
            }
        });
}

fn abbreviate(name: &str) -> String {
    let mut out = String::new();
    for part in name.split_whitespace() {
        if let Some(c) = part.chars().next() {
            out.push(c.to_ascii_uppercase());
        }
        if out.chars().count() >= 2 {
            break;
        }
    }
    if out.is_empty() {
        "?".to_owned()
    } else {
        out
    }
}
