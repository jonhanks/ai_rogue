use eframe::egui;

mod game_condition;
mod item;
mod npc;
mod state;
use game_condition::{GameStatus, TreasureHuntCondition, SurvivalCondition, CollectionCondition};
use item::ItemType;
use npc::NPCType;
use state::{GameState, TileType, WorldItem};

#[derive(Default, PartialEq)]
pub enum DialogState {
    #[default]
    GameTypeSelection,
    NoDialog,
    QuitConfirmation,
    UseItem,
    GameOver,
    Victory,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AvailableGameType {
    TreasureHunt,
    Survival,
    Collection,
}

#[derive(Debug, Default)]
pub struct WorldViewInteraction {
    pub mouse_position: Option<(i32, i32)>,
    pub clicked_position: Option<(i32, i32)>,
}

impl WorldViewInteraction {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_mouse_position(mut self, pos: Option<(i32, i32)>) -> Self {
        self.mouse_position = pos;
        self
    }
    
    pub fn with_clicked_position(mut self, pos: Option<(i32, i32)>) -> Self {
        self.clicked_position = pos;
        self
    }
}

impl AvailableGameType {
    pub fn get_name(&self) -> &str {
        match self {
            AvailableGameType::TreasureHunt => "Treasure Hunt",
            AvailableGameType::Survival => "Survival Challenge",
            AvailableGameType::Collection => "Item Collection",
        }
    }

    pub fn get_description(&self) -> &str {
        match self {
            AvailableGameType::TreasureHunt => "Find and collect the treasure while avoiding dangers.",
            AvailableGameType::Survival => "Survive for 50 turns without dying.",
            AvailableGameType::Collection => "Collect 3 gems, 2 scrolls, and 1 potion.",
        }
    }
}

pub struct RoguelikeApp {
    game_state: Option<GameState>,
    dialog_state: DialogState,
    mouse_world_pos: Option<(i32, i32)>,
}

impl RoguelikeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_style
        Self {
            game_state: None,
            dialog_state: DialogState::GameTypeSelection,
            mouse_world_pos: None,
        }
    }
}

impl eframe::App for RoguelikeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Handle input
        self.handle_input(ctx);

        // Check game status using the new condition system
        if self.dialog_state == DialogState::NoDialog {
            if let Some(ref game_state) = self.game_state {
                match game_state.check_game_status() {
                    GameStatus::Lost => {
                        self.dialog_state = DialogState::GameOver;
                    }
                    GameStatus::Won => {
                        self.dialog_state = DialogState::Victory;
                    }
                    GameStatus::Playing => {
                        // Continue playing
                    }
                }
            }
        }

        // Show appropriate dialog
        match self.dialog_state {
            DialogState::GameTypeSelection => {
                self.show_game_type_selection_dialog(ctx, frame);
                return; // Don't process anything else until game type is selected
            }
            DialogState::GameOver => {
                self.show_game_over_dialog(ctx, frame);
                return; // Don't process anything else if game is over
            }
            DialogState::Victory => {
                self.show_victory_dialog_window(ctx, frame);
                return; // Don't process anything else if player won
            }
            DialogState::QuitConfirmation => {
                self.show_quit_confirmation_dialog(ctx, frame);
            }
            DialogState::UseItem => {
                self.show_use_item_dialog_window(ctx, frame);
            }
            DialogState::NoDialog => {
                // Continue with normal game processing
            }
        }

        // Main UI layout - only show if game is initialized
        if let Some(ref game_state) = self.game_state {
            let mut world_interaction = WorldViewInteraction::new();
            
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
                                world_interaction = self.draw_world_view(ui, game_state);
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
                            self.draw_info_panel(ui, game_state);
                        },
                    );
                });
            });
            
            // Update mouse position based on interaction
            self.mouse_world_pos = world_interaction.mouse_position;
        }
    }
}

impl RoguelikeApp {
    fn handle_input(&mut self, ctx: &egui::Context) {
        // Only handle input if game is initialized
        if let Some(ref mut game_state) = self.game_state {
            // Add death message if player just died
            if !game_state.player.is_alive() {
                game_state.add_log_message("Your character has met its end...".to_string());
            }
        }

        // Handle keyboard input for movement and quit
        ctx.input(|i| {
            // Check for quit key first
            if i.key_pressed(egui::Key::Q) {
                self.dialog_state = DialogState::QuitConfirmation;
                return;
            }

            // Only handle movement and commands if no dialog is shown and game is initialized
            if self.dialog_state == DialogState::NoDialog {
                if let Some(ref mut game_state) = self.game_state {
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

                    let mut player_acted = false;

                    // Try to move the player
                    if dx != 0 || dy != 0 {
                        game_state.try_move_player(dx, dy);
                        player_acted = true;
                    }

                    // Check for pickup command
                    if i.key_pressed(egui::Key::P) {
                        game_state.try_pickup_item();
                        player_acted = true;
                    }

                    // Check for use item command
                    if i.key_pressed(egui::Key::U) {
                        if !game_state.player.inventory.is_empty() {
                            self.dialog_state = DialogState::UseItem;
                        } else {
                            game_state.add_log_message("You have no items to use.".to_string());
                        }
                        player_acted = true;
                    }

                    // Process NPC actions after player acts
                    if player_acted {
                        game_state.process_npc_actions();
                    }
                }
            }
        });
    }

    fn show_game_type_selection_dialog(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Select Game Type")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label("Choose your adventure:");
                    ui.add_space(20.0);

                    let game_types = vec![
                        AvailableGameType::TreasureHunt,
                        AvailableGameType::Survival,
                        AvailableGameType::Collection,
                    ];

                    for game_type in game_types {
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.strong(game_type.get_name());
                                ui.label(game_type.get_description());
                                ui.add_space(5.0);
                                if ui.button("Play this mode").clicked() {
                                    self.start_game_with_type(game_type);
                                }
                            });
                        });
                        ui.add_space(10.0);
                    }
                    
                    ui.add_space(10.0);
                });
            });
    }

    fn start_game_with_type(&mut self, game_type: AvailableGameType) {
        let game_condition: Box<dyn game_condition::GameCondition> = match game_type {
            AvailableGameType::TreasureHunt => Box::new(TreasureHuntCondition),
            AvailableGameType::Survival => Box::new(SurvivalCondition::new(50)),
            AvailableGameType::Collection => Box::new(CollectionCondition::new(vec![
                (ItemType::Gem, 3),
                (ItemType::Scroll, 2),
                (ItemType::Potion, 1),
            ])),
        };

        self.game_state = Some(GameState::with_condition(game_condition));
        self.dialog_state = DialogState::NoDialog;
    }

    fn show_quit_confirmation_dialog(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                            self.dialog_state = DialogState::NoDialog;
                        }
                        ui.add_space(20.0);
                    });
                    ui.add_space(10.0);
                });
            });
    }

    fn show_game_over_dialog(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Game Over")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label("Your character has met its end!");
                    ui.label("Game Over");
                    ui.add_space(20.0);
                    
                    if ui.button("Ok").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    
                    ui.add_space(10.0);
                });
            });
    }

    fn show_use_item_dialog_window(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(ref mut game_state) = self.game_state {
            egui::Window::new("Use Item")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.add_space(10.0);
                        ui.label("Choose an item to use:");
                        ui.add_space(10.0);

                        let mut item_to_use: Option<usize> = None;

                        // Show each item in inventory as a button
                        for (index, item) in game_state.player.inventory.iter().enumerate() {
                            if ui.button(&item.label).clicked() {
                                item_to_use = Some(index);
                            }
                        }

                        ui.add_space(10.0);

                        // Cancel button
                        if ui.button("Cancel").clicked() {
                            self.dialog_state = DialogState::NoDialog;
                        }

                        // Handle item usage
                        if let Some(index) = item_to_use {
                            let item = game_state.player.inventory.remove(index);
                            let result = game_state.use_item(item);
                            
                            // Handle the result
                            if let Some(returned_item) = result.returned_to_inventory {
                                game_state.player.inventory.push(returned_item);
                            }
                            
                            for dropped_item in result.dropped_on_ground {
                                game_state.world.items.push(WorldItem::new(
                                    game_state.player.position.0,
                                    game_state.player.position.1,
                                    dropped_item
                                ));
                            }
                            
                            // Process NPC actions after item use
                            game_state.process_npc_actions();
                            
                            self.dialog_state = DialogState::NoDialog;
                        }

                        ui.add_space(10.0);
                    });
                });
        }
    }

    fn show_victory_dialog_window(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Victory!")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    ui.label("Congratulations!");
                    
                    let victory_message = if let Some(ref game_state) = self.game_state {
                        game_state.get_victory_message()
                    } else {
                        "Congratulations, you are surrounded by adoring masses chanting your name and cheering your victory! If only you knew how you won!"
                    };
                    ui.label(victory_message);
                    ui.add_space(20.0);
                    
                    if ui.button("Ok").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    
                    ui.add_space(10.0);
                });
            });
    }

    fn draw_world_view(&self, ui: &mut egui::Ui, game_state: &GameState) -> WorldViewInteraction {
        let mut interaction = WorldViewInteraction::new();
        let available_size = ui.available_size();

        // Draw the game world
        ui.allocate_ui_with_layout(
            available_size,
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                ui.label(format!("GOAL: {}", game_state.get_win_description()));
                ui.separator();
                ui.label(format!("World Size: {}x{}", game_state.world.size.0, game_state.world.size.1));
                ui.label(format!("Player Position: ({}, {})", game_state.player.position.0, game_state.player.position.1));
                ui.label(format!("Floor: {}", game_state.world.current_floor));
                if let Some((x, y)) = self.mouse_world_pos {
                    ui.label(format!("Mouse Over: ({}, {})", x, y));
                } else {
                    ui.label("Mouse Over: --");
                }

                // World representation that takes remaining space
                let visible_width = game_state.world.size.0.min(60);
                let visible_height = game_state.world.size.1.min(30);
                
                egui::ScrollArea::both()
                    .max_height(ui.available_height())
                    .show(ui, |ui| {
                        ui.style_mut().override_font_id = Some(egui::FontId::monospace(12.0));
                        ui.style_mut().spacing.item_spacing = egui::Vec2::new(0.0, 0.0);

                        for y in 0..visible_height {
                            ui.horizontal(|ui| {
                                ui.style_mut().spacing.item_spacing = egui::Vec2::new(0.0, 0.0);
                                
                                for x in 0..visible_width {
                                    let (tile_char, color) = if x == game_state.player.position.0 as usize &&
                                        y == game_state.player.position.1 as usize {
                                        ('@', (255, 255, 0)) // Player - bright yellow
                                    } else if let Some(npc) = game_state.npcs.iter().find(|npc| 
                                        npc.position.0 == x as i32 && npc.position.1 == y as i32) {
                                        npc.display_info()
                                    } else if let Some(world_item) = game_state.world.items.iter().find(|item| 
                                        item.position.0 == x as i32 && item.position.1 == y as i32) {
                                        world_item.item.display_info()
                                    } else {
                                        match game_state.world.get_tile(x as i32, y as i32) {
                                            Some(tile) => tile.display_info(),
                                            None => (' ', (0, 0, 0)),
                                        }
                                    };
                                    
                                    let label = egui::Label::new(
                                        egui::RichText::new(tile_char.to_string())
                                            .color(egui::Color32::from_rgb(color.0, color.1, color.2))
                                    ).sense(egui::Sense::hover());
                                    let response = ui.add(label);
                                    
                                    if response.hovered() {
                                        interaction.mouse_position = Some((x as i32, y as i32));
                                    }
                                }
                            });
                        }
                    });
            },
        );
        
        interaction
    }

    fn draw_info_panel(&self, ui: &mut egui::Ui, game_state: &GameState) {
        ui.group(|ui| {
            ui.label("Player Stats");
            ui.separator();

            ui.label(format!("Level: {}", game_state.player.level));
            ui.label(format!("Health: {}/{}", game_state.player.health, game_state.player.max_health));
            ui.label(format!("Experience: {}", game_state.player.experience));
            ui.label(format!("Floor: {}", game_state.world.current_floor));
            ui.label(format!("Position: ({}, {})", game_state.player.position.0, game_state.player.position.1));
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Inventory");
            ui.separator();
            if game_state.player.inventory.is_empty() {
                ui.label("Empty");
            } else {
                for item in &game_state.player.inventory {
                    ui.label(&item.label);
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
                    for message in &game_state.log_messages {
                        ui.label(message);
                    }
                });
        });

        ui.add_space(10.0);

        // Show hover description if mouse is over a map position
        if self.mouse_world_pos.is_some() {
            self.draw_hover_description(ui, game_state);
            ui.add_space(10.0);
        }

        ui.group(|ui| {
            ui.label("Controls");
            ui.separator();
            ui.label("Arrow Keys / WASD: Move");
            ui.label("P: Pick up item");
            ui.label("U: Use item");
            ui.label("Q: Quit");
            ui.label("More controls coming...");
        });
    }

    fn draw_hover_description(&self, ui: &mut egui::Ui, game_state: &GameState) {
        if let Some((hover_x, hover_y)) = self.mouse_world_pos {
            ui.group(|ui| {
                ui.label("Location Details");
                ui.separator();
                
                // Check what's at this position
                let mut descriptions = Vec::new();
                
                // Check if player is here
                if game_state.player.position.0 == hover_x && 
                   game_state.player.position.1 == hover_y {
                    descriptions.push("Player (@) is here".to_string());
                }
                
                // Check for NPCs
                if let Some(npc) = game_state.npcs.iter().find(|npc| 
                    npc.position.0 == hover_x && npc.position.1 == hover_y) {
                    descriptions.push(format!("{} ({}) - {}", npc.name, npc.get_display_char(), 
                        match npc.npc_type {
                            NPCType::Goblin => "A mischievous goblin",
                            NPCType::Orc => "A fierce orc warrior",
                            NPCType::Skeleton => "Ancient bones animated by dark magic",
                            NPCType::Merchant => "A traveling merchant",
                            NPCType::Guard => "A stalwart guard",
                        }));
                }
                
                // Check for items
                if let Some(world_item) = game_state.world.items.iter().find(|item| 
                    item.position.0 == hover_x && item.position.1 == hover_y) {
                    descriptions.push(format!("{} ({}) - {}", 
                        world_item.item.label, 
                        world_item.item.get_display_char(), 
                        world_item.item.description));
                }
                
                // Check tile type
                if let Some(tile) = game_state.world.get_tile(hover_x, hover_y) {
                    let tile_desc = match tile {
                        TileType::Wall => "Solid stone wall",
                        TileType::Floor => "Stone floor",
                        TileType::Door => "Wooden door",
                        TileType::Stairs => "Stone stairs",
                        TileType::Empty => "Empty space",
                    };
                    descriptions.push(format!("Terrain: {} ({})", tile_desc, 
                        match tile {
                            TileType::Wall => '#',
                            TileType::Floor => '.',
                            TileType::Door => '+',
                            TileType::Stairs => '>',
                            TileType::Empty => ' ',
                        }));
                }
                
                ui.label(format!("Position: ({}, {})", hover_x, hover_y));
                ui.separator();
                
                if descriptions.is_empty() {
                    ui.label("Nothing of interest here.");
                } else {
                    for desc in descriptions {
                        ui.label(desc);
                    }
                }
            });
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