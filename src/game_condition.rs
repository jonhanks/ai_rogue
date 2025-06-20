use crate::item::{Item, ItemType};
use crate::npc::{NPC, NPCType};
use crate::state::{GameState, WorldItem};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum GameStatus {
    Playing,
    Won,
    Lost,
}

/// Trait for determining win/loss conditions in the game
pub trait GameCondition {
    /// Check the current game status based on game state
    fn check_status(&self, game_state: &GameState) -> GameStatus;
    
    /// Get a description of the win condition for this game type
    fn win_description(&self) -> String;
    
    /// Get a description of the loss condition for this game type
    fn loss_description(&self) -> &str;
    
    /// Get the victory message shown when the player wins
    fn victory_message(&self) -> &str;
    
    /// Setup the world and NPCs for this game mode
    fn setup_world(&self, world: &mut crate::state::GameWorld, npcs: &mut Vec<crate::npc::NPC>, player: &mut crate::state::Player);
}

/// Default treasure hunt game condition
/// Win: Collect the treasure
/// Lose: Player dies (health <= 0)
#[derive(Debug)]
pub struct TreasureHuntCondition;

impl GameCondition for TreasureHuntCondition {
    fn check_status(&self, game_state: &GameState) -> GameStatus {
        // Check loss condition first
        if !game_state.player.is_alive() {
            return GameStatus::Lost;
        }
        
        // Check win condition - player has treasure in inventory
        if game_state.player.inventory.iter().any(|item| item.item_type == ItemType::Treasure) {
            return GameStatus::Won;
        }
        
        GameStatus::Playing
    }
    
    fn win_description(&self) -> String {
        "Find and collect the treasure!".to_string()
    }
    
    fn loss_description(&self) -> &str {
        "Don't let your health reach zero!"
    }
    
    fn victory_message(&self) -> &str {
        "Congratulations! You have found the treasure and escaped the dungeon!"
    }
    
    fn setup_world(&self, world: &mut crate::state::GameWorld, npcs: &mut Vec<crate::npc::NPC>, player: &mut crate::state::Player) {
        // Default setup for treasure hunt - variety of NPCs
        npcs.push(NPC::new(5, 5, NPCType::Goblin, "Grob".to_string()));
        npcs.push(NPC::new(15, 8, NPCType::Merchant, "The Merchant".to_string()));
        npcs.push(NPC::new(25, 12, NPCType::Skeleton, "Bonecrusher".to_string()));
        npcs.push(NPC::new(8, 20, NPCType::Guard, "Guard Captain".to_string()));
        npcs.push(NPC::new(30, 25, NPCType::Orc, "Orc Warrior".to_string()));

        // Add treasure chest at a specific location
        let treasure_chest = Item::new(
            ItemType::TreasureChest,
            "Treasure Chest".to_string(),
            "A mysterious chest that might contain valuable items.".to_string(),
        );
        world.items.push(WorldItem::new(35, 18, treasure_chest));
        
        // Set default player position
        player.position = (10, 15);
    }
}

/// Survival game condition
/// Win: Survive for a certain number of turns
/// Lose: Player dies
#[derive(Debug)]
pub struct SurvivalCondition {
    pub target_turns: u32,
}

impl SurvivalCondition {
    pub fn new(target_turns: u32) -> Self {
        Self { target_turns }
    }
}

impl GameCondition for SurvivalCondition {
    fn check_status(&self, game_state: &GameState) -> GameStatus {
        // Check loss condition first
        if !game_state.player.is_alive() {
            return GameStatus::Lost;
        }
        
        // Check win condition - survived enough turns
        // Note: We'd need to add a turn counter to GameState for this to work
        // For now, this is just a placeholder implementation
        if game_state.log_messages.len() >= self.target_turns as usize {
            return GameStatus::Won;
        }
        
        GameStatus::Playing
    }
    
    fn win_description(&self) -> String {
        format!("Survive for {} turns!", self.target_turns)
    }
    
    fn loss_description(&self) -> &str {
        "Don't let your health reach zero!"
    }
    
    fn victory_message(&self) -> &str {
        "Amazing! You have survived the required number of turns and proven your resilience!"
    }
    
    fn setup_world(&self, world: &mut crate::state::GameWorld, npcs: &mut Vec<crate::npc::NPC>, player: &mut crate::state::Player) {
        let mut rng = rand::thread_rng();
        let mut occupied_positions = Vec::new();
        
        // Helper function to find a random valid position
        let mut find_random_position = || {
            for _ in 0..100 { // Try up to 100 times to find a valid position
                let x = rng.gen_range(1..world.size.0 as i32 - 1);
                let y = rng.gen_range(1..world.size.1 as i32 - 1);
                
                if world.is_walkable(x, y) && !occupied_positions.contains(&(x, y)) {
                    occupied_positions.push((x, y));
                    return Some((x, y));
                }
            }
            None // Failed to find position
        };
        
        // Place player randomly
        if let Some(pos) = find_random_position() {
            player.position = pos;
        } else {
            player.position = (10, 15); // Fallback position
        }
        
        // Survival mode - 5 aggressive orcs at random positions
        let orc_names = [
            "Urg the Destroyer",
            "Grok the Fierce", 
            "Morg the Brutal",
            "Thok the Savage",
            "Vrak the Terrible"
        ];
        
        for name in orc_names.iter() {
            if let Some(pos) = find_random_position() {
                npcs.push(NPC::new(pos.0, pos.1, NPCType::Orc, name.to_string()));
            }
        }
    }
}

/// Collection game condition
/// Win: Collect a certain number of items of specific types
/// Lose: Player dies
#[derive(Debug)]
pub struct CollectionCondition {
    pub required_items: Vec<(ItemType, u32)>, // (item_type, quantity)
}

impl CollectionCondition {
    pub fn new(required_items: Vec<(ItemType, u32)>) -> Self {
        Self { required_items }
    }
}

impl GameCondition for CollectionCondition {
    fn check_status(&self, game_state: &GameState) -> GameStatus {
        // Check loss condition first
        if !game_state.player.is_alive() {
            return GameStatus::Lost;
        }
        
        // Check win condition - collected all required items
        for (required_type, required_count) in &self.required_items {
            let collected_count = game_state.player.inventory.iter()
                .filter(|item| item.item_type == *required_type)
                .count() as u32;
            
            if collected_count < *required_count {
                return GameStatus::Playing;
            }
        }
        
        GameStatus::Won
    }
    
    fn win_description(&self) -> String {
        "Collect all required items!".to_string()
    }
    
    fn loss_description(&self) -> &str {
        "Don't let your health reach zero!"
    }
    
    fn victory_message(&self) -> &str {
        "Excellent! You have collected all the required items and completed your quest!"
    }
    
    fn setup_world(&self, _world: &mut crate::state::GameWorld, npcs: &mut Vec<crate::npc::NPC>, player: &mut crate::state::Player) {
        // Collection mode - merchant who provides items plus some other NPCs
        npcs.push(NPC::new(25, 15, NPCType::Merchant, "The Wandering Merchant".to_string()));
        npcs.push(NPC::new(5, 5, NPCType::Goblin, "Snitch".to_string()));
        npcs.push(NPC::new(40, 20, NPCType::Guard, "Tower Guard".to_string()));
        npcs.push(NPC::new(15, 25, NPCType::Orc, "Grum the Collector".to_string()));
        
        // Set default player position
        player.position = (10, 15);
        
        // No initial items - the merchant will drop them
    }
}