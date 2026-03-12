use eframe::egui;
mod mock_server;
mod model;

mod ui_channels;
mod ui_chat;
mod ui_servers;

use model::{Channel, Server};

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "egui Discord Mock",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}

pub(crate) struct App {
    pub(crate) servers: Vec<Server>,
    pub(crate) selected_server: usize,
    pub(crate) selected_channel: usize,
    pub(crate) composer: String,
    pub(crate) scroll_to_bottom: bool,
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

    pub(crate) fn selected_channel_mut(&mut self) -> &mut Channel {
        let server_idx = self
            .selected_server
            .min(self.servers.len().saturating_sub(1));
        let channel_len = self.servers[server_idx].channels.len();
        let channel_idx = self.selected_channel.min(channel_len.saturating_sub(1));
        &mut self.servers[server_idx].channels[channel_idx]
    }

    pub(crate) fn selected_server(&self) -> &Server {
        let idx = self
            .selected_server
            .min(self.servers.len().saturating_sub(1));
        &self.servers[idx]
    }

    pub(crate) fn selected_channel(&self) -> &Channel {
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

        ui_servers::ui_servers(self, ctx);
        ui_channels::ui_channels(self, ctx);
        ui_chat::ui_chat(self, ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.vertical_centered(|ui| {
                    ui.strong("big");
                    ui.label("Hello, egui!");
                });
            });
        });
    }
}
