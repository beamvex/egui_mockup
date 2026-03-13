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
    pub(crate) show_dialog: bool,
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
            show_dialog: false,
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

        if self.show_dialog {
            egui::Window::new("Dialog")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.label("Button clicked!");
                    if ui.button("OK").clicked() {
                        self.show_dialog = false;
                    }
                });
        }

        //ui_servers::ui_servers(self, ctx);
        //ui_channels::ui_channels(self, ctx);
        //ui_chat::ui_chat(self, ctx);

        egui::CentralPanel::default()
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                let center = rect.center();

                let top = egui::Color32::from_rgb(10, 40, 22);
                let bottom = egui::Color32::from_rgb(3, 18, 10);

                let mut mesh = egui::epaint::Mesh::default();
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: rect.left_top(),
                    uv: egui::pos2(0.0, 0.0),
                    color: top,
                });
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: rect.right_top(),
                    uv: egui::pos2(0.0, 0.0),
                    color: top,
                });
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: rect.right_bottom(),
                    uv: egui::pos2(0.0, 0.0),
                    color: bottom,
                });
                mesh.vertices.push(egui::epaint::Vertex {
                    pos: rect.left_bottom(),
                    uv: egui::pos2(0.0, 0.0),
                    color: bottom,
                });
                mesh.indices.extend([0, 1, 2, 0, 2, 3]);
                ui.painter().add(egui::Shape::mesh(mesh));

                let font_big = egui::FontId::proportional(28.0);
                let font_normal = egui::FontId::proportional(16.0);
                let color = egui::Color32::WHITE;

                ui.painter().text(
                    center + egui::vec2(0.0, -142.0),
                    egui::Align2::RIGHT_CENTER,
                    "big",
                    font_big,
                    color,
                );
                ui.painter().text(
                    center + egui::vec2(0.0, 180.0),
                    egui::Align2::CENTER_CENTER,
                    "Hello, egui!",
                    font_normal,
                    color,
                );


                let mut widget = egui::Button::new("Click me");
                widget = widget.fill(egui::Color32::from_rgb(0, 120, 212));

                let button_rect =
                    egui::Rect::from_center_size(center + egui::vec2(0.0, 240.0), egui::vec2(140.0, 32.0));
                if ui.put(button_rect, widget).clicked() {
                    self.show_dialog = true;
                }
            });
    }
}
