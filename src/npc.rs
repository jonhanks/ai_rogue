use crate::item::Item;
use crate::state::{GameWorld, Player};

#[derive(Debug, Clone)]
pub struct NPC {
    pub position: (i32, i32),
    pub npc_type: NPCType,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NPCType {
    Goblin,
    Orc,
    Skeleton,
    Merchant,
    Guard,
}

#[derive(Debug)]
pub enum InteractionResult {
    Nothing,
    NPC(NPC),
    Item(Item),
}

impl NPC {
    pub fn new(x: i32, y: i32, npc_type: NPCType, name: String) -> Self {
        Self {
            position: (x, y),
            npc_type,
            name,
        }
    }

    pub fn get_display_char(&self) -> char {
        match self.npc_type {
            NPCType::Goblin => 'g',
            NPCType::Orc => 'O',
            NPCType::Skeleton => 'S',
            NPCType::Merchant => 'M',
            NPCType::Guard => 'G',
        }
    }

    pub fn display_info(&self) -> (char, (u8, u8, u8)) {
        let char = self.get_display_char();
        let color = match self.npc_type {
            NPCType::Goblin => (0, 255, 0), // Green
            NPCType::Orc => (180, 50, 50), // Dark red
            NPCType::Skeleton => (200, 200, 200), // Light gray
            NPCType::Merchant => (100, 150, 255), // Light blue
            NPCType::Guard => (70, 70, 150), // Dark blue
        };
        (char, color)
    }

    /// Perform an action for this NPC during the game turn
    /// For now, this is a placeholder that does nothing
    pub fn perform_action(&mut self, _world: &mut GameWorld, _player: &mut Player) -> Vec<String> {
        // Return empty vector for now - no log messages
        Vec::new()
    }
}