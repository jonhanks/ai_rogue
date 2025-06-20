use crate::item::{Item, ItemType};
use crate::state::{GameWorld, Player, WorldItem};
use rand::Rng;

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
    pub fn perform_action(&mut self, world: &mut GameWorld, player: &mut Player, other_npcs: &[NPC]) -> Vec<String> {
        let mut log_messages = Vec::new();
        
        match self.npc_type {
            NPCType::Merchant => {
                self.merchant_behavior(world, player, other_npcs, &mut log_messages);
            }
            _ => {
                // Other NPCs do nothing for now
            }
        }
        
        log_messages
    }
    
    /// Merchant-specific behavior: random movement and item interaction
    fn merchant_behavior(&mut self, world: &mut GameWorld, player: &Player, other_npcs: &[NPC], log_messages: &mut Vec<String>) {
        let mut rng = rand::thread_rng();
        
        // 24% chance to move each turn
        if rng.gen_range(0..100) < 24 {
            self.try_random_move(world, player, other_npcs, log_messages, &mut rng);
        }
    }
    
    /// Try to move the merchant randomly (up to 2 attempts)
    fn try_random_move(&mut self, world: &mut GameWorld, player: &Player, other_npcs: &[NPC], log_messages: &mut Vec<String>, rng: &mut impl Rng) {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]; // down, up, right, left
        
        // Try up to 2 times to find a valid move
        for _attempt in 0..2 {
            let (dx, dy) = directions[rng.gen_range(0..directions.len())];
            let new_pos = (self.position.0 + dx, self.position.1 + dy);
            
            // Check if the new position is valid and walkable
            if !world.is_valid_position(new_pos.0, new_pos.1) || !world.is_walkable(new_pos.0, new_pos.1) {
                continue; // Try another direction
            }
            
            // Check if player is at the new position
            if player.position == new_pos {
                continue; // Try another direction
            }
            
            // Check if another NPC is at the new position
            if other_npcs.iter().any(|npc| npc.position == new_pos) {
                continue; // Try another direction
            }
            
            // Valid move found! Check if there's an item at the new position - if so, destroy it
            if let Some(item_index) = world.items.iter().position(|item| item.position == new_pos) {
                let destroyed_item = world.items.remove(item_index);
                log_messages.push(format!("The merchant and his cart destroyed the {} on the ground!", destroyed_item.item.label));
            }
            
            // Move the merchant
            self.position = new_pos;
            
            // 15% chance to drop an item after moving
            if rng.gen_range(0..100) < 15 {
                self.drop_random_item(world, log_messages, rng);
            }
            
            return; // Successfully moved, exit the function
        }
        
        // If we get here, no valid move was found after 2 attempts
    }
    
    /// Drop a random collectible item
    fn drop_random_item(&self, world: &mut GameWorld, log_messages: &mut Vec<String>, rng: &mut impl Rng) {
        let item_types = [ItemType::Gem, ItemType::Scroll, ItemType::Potion];
        let item_type = item_types[rng.gen_range(0..item_types.len())].clone();
        
        let (name, description) = match item_type {
            ItemType::Gem => ("Precious Gem", "A sparkling gem that catches the light"),
            ItemType::Scroll => ("Ancient Scroll", "A scroll covered in mysterious writing"),
            ItemType::Potion => ("Magic Potion", "A bubbling potion with unknown effects"),
            _ => ("Unknown Item", "A mysterious object"),
        };
        
        let item = Item::new(item_type, name.to_string(), description.to_string());
        world.items.push(WorldItem::new(self.position.0, self.position.1, item));
        
        log_messages.push(format!("The merchant dropped a {} from his cart!", name));
    }
}