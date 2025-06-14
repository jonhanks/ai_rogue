use eframe::egui;

#[derive(Debug, Clone)]
pub struct Player {
    pub position: (i32, i32),
    pub health: i32,
    pub max_health: i32,
    pub level: i32,
    pub experience: i32,
    pub inventory: Vec<String>, // Placeholder for now
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: (10, 15),
            health: 100,
            max_health: 100,
            level: 1,
            experience: 0,
            inventory: Vec::new(),
        }
    }
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: (x, y),
            ..Default::default()
        }
    }

    pub fn move_to(&mut self, new_pos: (i32, i32)) {
        self.position = new_pos;
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}

#[derive(Debug, Clone)]
pub struct GameWorld {
    pub size: (usize, usize), // width, height
    pub current_floor: i32,
    pub tiles: Vec<Vec<TileType>>, // 2D grid of tiles
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    Door,
    Stairs,
    Empty,
}

impl Default for GameWorld {
    fn default() -> Self {
        let size = (50, 30);
        let mut tiles = vec![vec![TileType::Empty; size.1]; size.0];

        // Create a simple room with walls
        for x in 0..size.0 {
            for y in 0..size.1 {
                if x == 0 || x == size.0 - 1 || y == 0 || y == size.1 - 1 {
                    tiles[x][y] = TileType::Wall;
                } else if (x + y) % 7 == 0 {
                    tiles[x][y] = TileType::Floor;
                } else {
                    tiles[x][y] = TileType::Empty;
                }
            }
        }

        Self {
            size,
            current_floor: 1,
            tiles,
        }
    }
}

impl GameWorld {
    pub fn new(width: usize, height: usize) -> Self {
        let mut world = Self {
            size: (width, height),
            current_floor: 1,
            tiles: vec![vec![TileType::Empty; height]; width],
        };
        world.generate_simple_room();
        world
    }

    pub fn generate_simple_room(&mut self) {
        // Generate a simple room layout
        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                if x == 0 || x == self.size.0 - 1 || y == 0 || y == self.size.1 - 1 {
                    self.tiles[x][y] = TileType::Wall;
                } else if (x + y) % 7 == 0 {
                    self.tiles[x][y] = TileType::Floor;
                } else {
                    self.tiles[x][y] = TileType::Empty;
                }
            }
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&TileType> {
        if x >= 0 && y >= 0 && (x as usize) < self.size.0 && (y as usize) < self.size.1 {
            Some(&self.tiles[x as usize][y as usize])
        } else {
            None
        }
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        match self.get_tile(x, y) {
            Some(TileType::Floor) | Some(TileType::Door) | Some(TileType::Empty) => true,
            _ => false,
        }
    }

    pub fn is_valid_position(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.size.0 && (y as usize) < self.size.1
    }
}

#[derive(Default)]
pub struct GameState {
    pub player: Player,
    pub world: GameWorld,
    pub log_messages: Vec<String>,
    pub game_over: bool,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::default(),
            world: GameWorld::default(),
            log_messages: vec![
                "Welcome to the dungeon!".to_string(),
                "Press arrow keys to move.".to_string(),
                "Explore carefully...".to_string(),
            ],
            game_over: false,
        }
    }

    pub fn add_log_message(&mut self, message: String) {
        self.log_messages.push(message);

        // Keep only the last 50 messages
        if self.log_messages.len() > 50 {
            self.log_messages.remove(0);
        }
    }

    pub fn try_move_player(&mut self, dx: i32, dy: i32) -> bool {
        let new_pos = (self.player.position.0 + dx, self.player.position.1 + dy);

        if self.world.is_valid_position(new_pos.0, new_pos.1) &&
            self.world.is_walkable(new_pos.0, new_pos.1) {
            self.player.move_to(new_pos);
            self.add_log_message(format!("Moved to ({}, {})", new_pos.0, new_pos.1));
            true
        } else {
            self.add_log_message("Can't move there!".to_string());
            false
        }
    }
}

#[derive(Default)]
pub struct RoguelikeApp {
    game_state: GameState,
}

impl RoguelikeApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_style
        Self {
            game_state: GameState::new(),
        }
    }
}

impl eframe::App for RoguelikeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle input
        self.handle_input(ctx);

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
        // Handle keyboard input for movement
        ctx.input(|i| {
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