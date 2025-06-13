use eframe::egui;

#[derive(Default)]
pub struct RoguelikeApp {
    // Game state will go here
    world_size: (usize, usize), // width, height
    player_pos: (i32, i32),
    player_health: i32,
    player_level: i32,
    current_floor: i32,
    log_messages: Vec<String>,
}

impl RoguelikeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_style
        Self {
            world_size: (50, 30),
            player_pos: (10, 15),
            player_health: 100,
            player_level: 1,
            current_floor: 1,
            log_messages: vec![
                "Welcome to the dungeon!".to_string(),
                "Press arrow keys to move.".to_string(),
                "Explore carefully...".to_string(),
            ],
        }
    }
}

impl eframe::App for RoguelikeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle input
        self.handle_input(ctx);

        // Main UI layout
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // World view panel (left side, takes most space)
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width() * 0.75, ui.available_height()),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.group(|ui| {
                            ui.label("World View");
                            ui.separator();
                            self.draw_world_view(ui);
                        });
                    },
                );

                ui.separator();

                // Information panel (right side)
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), ui.available_height()),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        self.draw_info_panel(ui);
                    },
                );
            });
        });
    }
}

impl RoguelikeApp {
    fn handle_input(&mut self, ctx: &egui::Context) {
        // Handle keyboard input for movement
        ctx.input(|i| {
            let mut moved = false;
            let mut new_pos = self.player_pos;

            if i.key_pressed(egui::Key::ArrowUp) || i.key_pressed(egui::Key::W) {
                new_pos.1 -= 1;
                moved = true;
            }
            if i.key_pressed(egui::Key::ArrowDown) || i.key_pressed(egui::Key::S) {
                new_pos.1 += 1;
                moved = true;
            }
            if i.key_pressed(egui::Key::ArrowLeft) || i.key_pressed(egui::Key::A) {
                new_pos.0 -= 1;
                moved = true;
            }
            if i.key_pressed(egui::Key::ArrowRight) || i.key_pressed(egui::Key::D) {
                new_pos.0 += 1;
                moved = true;
            }

            // Bounds checking
            if moved {
                if new_pos.0 >= 0 && new_pos.0 < self.world_size.0 as i32 &&
                   new_pos.1 >= 0 && new_pos.1 < self.world_size.1 as i32 {
                    self.player_pos = new_pos;
                    self.add_log_message(format!("Moved to ({}, {})", new_pos.0, new_pos.1));
                }
            }
        });
    }

    fn draw_world_view(&mut self, ui: &mut egui::Ui) {
        let available_size = ui.available_size();
        
        // For now, we'll draw a simple grid representation
        // This is a placeholder - you'll replace this with your actual world rendering
        ui.allocate_ui_with_layout(
            available_size,
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.label(format!("World Size: {}x{}", self.world_size.0, self.world_size.1));
                ui.label(format!("Player Position: ({}, {})", self.player_pos.0, self.player_pos.1));
                
                // Simple ASCII-style representation
                egui::ScrollArea::both()
                    .max_height(available_size.y - 50.0)
                    .show(ui, |ui| {
                        ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));
                        
                        // Draw a simple representation of the world
                        let visible_width = 40;
                        let visible_height = 20;
                        
                        for y in 0..visible_height {
                            let mut row = String::new();
                            for x in 0..visible_width {
                                if x == self.player_pos.0 as usize && y == self.player_pos.1 as usize {
                                    row.push('@'); // Player
                                } else if x == 0 || x == visible_width - 1 || y == 0 || y == visible_height - 1 {
                                    row.push('#'); // Wall
                                } else if (x + y) % 7 == 0 {
                                    row.push('.'); // Floor
                                } else {
                                    row.push(' '); // Empty space
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
            
            ui.label(format!("Level: {}", self.player_level));
            ui.label(format!("Health: {}", self.player_health));
            ui.label(format!("Floor: {}", self.current_floor));
            ui.label(format!("Position: ({}, {})", self.player_pos.0, self.player_pos.1));
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Inventory");
            ui.separator();
            ui.label("Empty"); // Placeholder
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Message Log");
            ui.separator();
            
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for message in &self.log_messages {
                        ui.label(message);
                    }
                });
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Controls");
            ui.separator();
            ui.label("Arrow Keys / WASD: Move");
            ui.label("More controls coming...");
        });
    }

    fn add_log_message(&mut self, message: String) {
        self.log_messages.push(message);
        
        // Keep only the last 50 messages
        if self.log_messages.len() > 50 {
            self.log_messages.remove(0);
        }
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