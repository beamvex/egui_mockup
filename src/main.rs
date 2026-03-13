use eframe::egui;
mod mock_server;
mod model;

mod ui_channels;
mod ui_chat;
mod ui_servers;

use model::{Channel, Server};

#[derive(Clone, Default)]
struct Profile {
    display_name: String,
    email: String,
    telephone: String,
    dob: String,
    place_of_birth: String,
    favorite_word: String,
    address_line1: String,
    address_line2: String,
    city: String,
    state_or_province: String,
    postal_code: String,
    country: String,
    bio: String,
}

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
    profile: Profile,
    draft_profile: Profile,
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let servers = mock_server::mock_servers();
        let profile = Profile {
            display_name: "Robert".to_owned(),
            email: "robert@example.com".to_owned(),
            telephone: "".to_owned(),
            dob: "".to_owned(),
            place_of_birth: "".to_owned(),
            favorite_word: "".to_owned(),
            address_line1: "".to_owned(),
            address_line2: "".to_owned(),
            city: "".to_owned(),
            state_or_province: "".to_owned(),
            postal_code: "".to_owned(),
            country: "".to_owned(),
            bio: "".to_owned(),
        };
        Self {
            servers,
            selected_server: 0,
            selected_channel: 0,
            composer: String::new(),
            scroll_to_bottom: true,
            show_dialog: false,
            draft_profile: profile.clone(),
            profile,
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
            let mut open = true;
            egui::Window::new("Edit profile")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .open(&mut open)
                .show(ctx, |ui| {
                    ui.set_min_width(760.0);

                    let col_w = (ui.available_width() - 12.0) / 2.0;
                    let label_w = 120.0;
                    let col_layout = egui::Layout::top_down(egui::Align::Min);

                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("Display name"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.display_name)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("Email"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.email)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                    });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("Telephone"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.telephone)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("DOB"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.dob)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                    });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("Place of birth"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.place_of_birth)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("Fave word"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.favorite_word)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                    });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("Address line 1"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.address_line1)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("Address line 2"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.address_line2)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                    });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("City"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.city)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("State / Province"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.state_or_province)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                    });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("Postal code"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.postal_code)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                        ui.allocate_ui_with_layout(egui::vec2(col_w, 0.0), col_layout, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized([label_w, 0.0], egui::Label::new("Country"));
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.draft_profile.country)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                        });
                    });

                    ui.add_space(8.0);
                    ui.label("Bio");
                    ui.add(
                        egui::TextEdit::multiline(&mut self.draft_profile.bio)
                            .desired_rows(4)
                            .desired_width(f32::INFINITY),
                    );

                    ui.add_space(12.0);
                    ui.horizontal(|ui| {
                        let save = ui.button("Save").clicked();
                        let cancel = ui.button("Cancel").clicked();

                        if save {
                            self.profile = self.draft_profile.clone();
                            self.show_dialog = false;
                        }
                        if cancel {
                            self.draft_profile = self.profile.clone();
                            self.show_dialog = false;
                        }
                    });
                });

            if !open {
                self.draft_profile = self.profile.clone();
                self.show_dialog = false;
            }
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
                    self.draft_profile = self.profile.clone();
                    self.show_dialog = true;
                }
            });
    }
}
