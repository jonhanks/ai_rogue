use eframe::egui;

mod state;
use state::{GameState, TileType, NPC, NPCType};

#[derive(Default)]
pub struct RoguelikeApp {
    game_state: GameState,
    show_quit_dialog: bool,
}

impl RoguelikeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_style
        Self {
            game_state: GameState::new(),
            show_quit_dialog: false,
        }
    }
}

impl eframe::App for RoguelikeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Handle input
        self.handle_input(ctx);

        // Show quit confirmation dialog if needed
        if self.show_quit_dialog {
            self.show_quit_confirmation_dialog(ctx, frame);
        }

        // Main UI layout
        egui::CentralPanel::default().show(ctx, |ui| {
            let desired_height = ui.available_height();
            ui.horizontal(|ui| {
                // World view panel (left side, takes most space and full height)
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width() * 0.75, desired_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.group(|ui| {
                            ui.set_height(ui.available_height());
                            ui.label("World View");
                            ui.separator();
                            self.draw_world_view(ui);
                        });
                    },
                );

                ui.separator();

                // Information panel (right side, full height)
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), ui.available_height()),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_height(ui.available_height());
                        self.draw_info_panel(ui);
                    },
                );
            });
        });
    }
}

impl RoguelikeApp {
    fn handle_input(&mut self, ctx: &egui::Context) {
        // Handle keyboard input for movement and quit
        ctx.input(|i| {
            // Check for quit key first
            if i.key_pressed(egui::Key::Q) {
                self.show_quit_dialog = true;
                return;
            }

            // Only handle movement if quit dialog is not shown
            if !self.show_quit_dialog {
                let mut dx = 0;
                let mut dy = 0;

                if i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W) {
                    dy = -1;
                }
                if i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S) {
                    dy = 1;
                }
                if i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A) {
                    dx = -1;
                }
                if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D) {
                    dx = 1;
                }

                // Try to move the player
                if dx != 0 || dy != 0 {
                    self.game_state.try_move_player(dx, dy);
                }
            }
        });
    }

    fn show_quit_confirmation_dialog(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::Window::new("Quit Game")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label("Are you sure you want to quit?");
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        ui.add_space(20.0);
                        if ui.button("Yes").clicked() {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        ui.add_space(20.0);
                        if ui.button("No").clicked() {
                            self.show_quit_dialog = false;
                        }
                        ui.add_space(20.0);
                    });
                    ui.add_space(10.0);
                });
            });
    }

    fn draw_world_view(&mut self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();

        // Draw the game world
        ui.allocate_ui_with_layout(
            available_size,
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.label(format!("World Size: {}x{}", self.game_state.world.size.0, self.game_state.world.size.1));
                ui.label(format!("Player Position: ({}, {})", self.game_state.player.position.0, self.game_state.player.position.1));
                ui.label(format!("Floor: {}", self.game_state.world.current_floor));

                // World representation that takes remaining space
                egui::ScrollArea::both()
                    .max_height(ui.available_height())
                    .show(ui, |ui| {
                        ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));

                        // Draw the world using the tile system
                        let visible_width = self.game_state.world.size.0.min(60);
                        let visible_height = self.game_state.world.size.1.min(30);

                        for y in 0..visible_height {
                            let mut row = String::new();
                            for x in 0..visible_width {
                                if x == self.game_state.player.position.0 as usize &&
                                    y == self.game_state.player.position.1 as usize {
                                    row.push('@'); // Player
                                } else if let Some(npc) = self.game_state.npcs.iter().find(|npc| 
                                    npc.position.0 == x as i32 && npc.position.1 == y as i32) {
                                    row.push(npc.get_display_char()); // NPC
                                } else {
                                    let tile_char = match self.game_state.world.get_tile(x as i32, y as i32) {
                                        Some(TileType::Wall) => '#',
                                        Some(TileType::Floor) => '.',
                                        Some(TileType::Door) => '+',
                                        Some(TileType::Stairs) => '>',
                                        Some(TileType::Empty) => ' ',
                                        None => ' ',
                                    };
                                    row.push(tile_char);
                                }
                            }
                            ui.label(row);
                        }
                    });
            },
        );
    }

    fn draw_info_panel(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label("Player Stats");
            ui.separator();

            ui.label(format!("Level: {}", self.game_state.player.level));
            ui.label(format!("Health: {}/{}", self.game_state.player.health, self.game_state.player.max_health));
            ui.label(format!("Experience: {}", self.game_state.player.experience));
            ui.label(format!("Floor: {}", self.game_state.world.current_floor));
            ui.label(format!("Position: ({}, {})", self.game_state.player.position.0, self.game_state.player.position.1));
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Inventory");
            ui.separator();
            if self.game_state.player.inventory.is_empty() {
                ui.label("Empty");
            } else {
                for item in &self.game_state.player.inventory {
                    ui.label(item);
                }
            }
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Message Log");
            ui.separator();

            egui::ScrollArea::vertical()
                .max_height(200.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for message in &self.game_state.log_messages {
                        ui.label(message);
                    }
                });
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Controls");
            ui.separator();
            ui.label("Arrow Keys / WASD: Move");
            ui.label("Q: Quit");
            ui.label("More controls coming...");
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Roguelike Game"),
        ..Default::default()
    };

    eframe::run_native(
        "Roguelike Game",
        options,
        Box::new(|cc| Ok(Box::new(RoguelikeApp::new(cc)))),
    )
}