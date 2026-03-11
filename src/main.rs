use eframe::egui;
mod mock_server;
mod model;

use model::{Server, Channel, Message};

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui Discord Mock",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}


struct App {
    servers: Vec<Server>,
    selected_server: usize,
    selected_channel: usize,
    composer: String,
    scroll_to_bottom: bool,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let servers = mock_server::mock_servers();
        Self {
            servers,
            selected_server: 0,
            selected_channel: 0,
            composer: String::new(),
            scroll_to_bottom: true,
        }
    }

    fn selected_channel_mut(&mut self) -> &mut Channel {
        let server_idx = self
            .selected_server
            .min(self.servers.len().saturating_sub(1));
        let channel_len = self.servers[server_idx].channels.len();
        let channel_idx = self.selected_channel.min(channel_len.saturating_sub(1));
        &mut self.servers[server_idx].channels[channel_idx]
    }

    fn selected_server(&self) -> &Server {
        let idx = self
            .selected_server
            .min(self.servers.len().saturating_sub(1));
        &self.servers[idx]
    }

    fn selected_channel(&self) -> &Channel {
        let server_idx = self
            .selected_server
            .min(self.servers.len().saturating_sub(1));
        let channel_len = self.servers[server_idx].channels.len();
        let channel_idx = self.selected_channel.min(channel_len.saturating_sub(1));
        &self.servers[server_idx].channels[channel_idx]
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());

        self.ui_servers(ctx);
        self.ui_channels(ctx);
        self.ui_chat(ctx);
    }
}

impl App {
    fn ui_servers(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("servers")
            .exact_width(64.0)
            .resizable(false)
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(0.0, 8.0);
                ui.add_space(8.0);

                for (i, server) in self.servers.iter().enumerate() {
                    let selected = i == self.selected_server;
                    let label = abbreviate(&server.name);
                    let button = egui::SelectableLabel::new(selected, label);
                    let resp = ui.add_sized([48.0, 48.0], button);
                    if resp.clicked() {
                        self.selected_server = i;
                        self.selected_channel = 0;
                        self.scroll_to_bottom = true;
                    }
                    resp.on_hover_text(&server.name);
                }
            });
    }

    fn ui_channels(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("channels")
            .default_width(220.0)
            .min_width(180.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.heading(self.selected_server().name.clone());
                ui.add_space(8.0);
                ui.separator();

                let channels = self.selected_server().channels.clone();
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for (i, ch) in channels.iter().enumerate() {
                            let selected = i == self.selected_channel;
                            let label = format!("# {}", ch.name);
                            if ui
                                .add(egui::SelectableLabel::new(selected, label))
                                .clicked()
                            {
                                self.selected_channel = i;
                                self.scroll_to_bottom = true;
                            }
                        }
                    });
            });
    }

    fn ui_chat(&mut self, ctx: &egui::Context) {
        let channel_name = self.selected_channel().name.clone();

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
                        egui::TextEdit::singleline(&mut self.composer)
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
            let messages = self.selected_channel().messages.clone();
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
            let trimmed = self.composer.trim();
            if !trimmed.is_empty() {
                let msg = Message {
                    author: "You".to_owned(),
                    content: trimmed.to_owned(),
                    timestamp: "now".to_owned(),
                };
                self.selected_channel_mut().messages.push(msg);
                self.composer.clear();
                self.scroll_to_bottom = true;
            }
        }

        if self.scroll_to_bottom {
            ctx.request_repaint();
            self.scroll_to_bottom = false;
        }
    }
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

