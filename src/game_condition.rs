use crate::item::ItemType;
use crate::state::GameState;

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
    fn win_description(&self) -> &str;
    
    /// Get a description of the loss condition for this game type
    fn loss_description(&self) -> &str;
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
    
    fn win_description(&self) -> &str {
        "Find and collect the treasure!"
    }
    
    fn loss_description(&self) -> &str {
        "Don't let your health reach zero!"
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
    
    fn win_description(&self) -> &str {
        "Survive for the required number of turns!"
    }
    
    fn loss_description(&self) -> &str {
        "Don't let your health reach zero!"
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
    
    fn win_description(&self) -> &str {
        "Collect all required items!"
    }
    
    fn loss_description(&self) -> &str {
        "Don't let your health reach zero!"
    }
}