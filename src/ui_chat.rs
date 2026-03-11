use eframe::egui;

use crate::model::Message;
use crate::App;

pub fn ui_chat(app: &mut App, ctx: &egui::Context) {
    let channel_name = app.selected_channel().name.clone();

    egui::TopBottomPanel::top("chat_header").show(ctx, |ui| {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.heading(format!("# {}", channel_name));
            ui.add_space(8.0);
            ui.weak("Mock Discord-like client");
        });
        ui.add_space(6.0);
        ui.separator();
    });

    let mut send_now = false;
    egui::TopBottomPanel::bottom("composer")
        .resizable(false)
        .show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                let input = ui.add(
                    egui::TextEdit::singleline(&mut app.composer)
                        .hint_text("Message #channel")
                        .desired_width(f32::INFINITY),
                );
                if input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    send_now = true;
                }
                if ui.button("Send").clicked() {
                    send_now = true;
                }
            });
            ui.add_space(6.0);
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        let messages = app.selected_channel().messages.clone();
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.add_space(8.0);
                for msg in messages {
                    ui.horizontal_wrapped(|ui| {
                        ui.strong(&msg.author);
                        ui.add_space(6.0);
                        ui.weak(msg.timestamp);
                    });
                    ui.label(msg.content);
                    ui.add_space(10.0);
                }
            });
    });

    if send_now {
        let trimmed = app.composer.trim();
        if !trimmed.is_empty() {
            let msg = Message {
                author: "You".to_owned(),
                content: trimmed.to_owned(),
                timestamp: "now".to_owned(),
            };
            app.selected_channel_mut().messages.push(msg);
            app.composer.clear();
            app.scroll_to_bottom = true;
        }
    }

    if app.scroll_to_bottom {
        ctx.request_repaint();
        app.scroll_to_bottom = false;
    }
}
