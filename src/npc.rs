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
            NPCType::Orc => {
                self.orc_behavior(world, player, other_npcs, &mut log_messages);
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
    
    /// Orc-specific behavior: aggressive movement towards player
    fn orc_behavior(&mut self, world: &mut GameWorld, player: &mut Player, other_npcs: &[NPC], log_messages: &mut Vec<String>) {
        let player_distance = self.distance_to_player(player);
        
        if player_distance <= 5.0 {
            // Close to player - move towards them or attack
            self.move_towards_player_or_attack(world, player, other_npcs, log_messages);
        } else {
            // Far from player - move randomly
            let mut rng = rand::thread_rng();
            self.try_random_move_orc(world, player, other_npcs, &mut rng);
        }
    }
    
    /// Calculate distance to player
    fn distance_to_player(&self, player: &Player) -> f32 {
        let dx = (self.position.0 - player.position.0) as f32;
        let dy = (self.position.1 - player.position.1) as f32;
        (dx * dx + dy * dy).sqrt()
    }
    
    /// Move towards player or attack if adjacent
    fn move_towards_player_or_attack(&mut self, world: &mut GameWorld, player: &mut Player, other_npcs: &[NPC], log_messages: &mut Vec<String>) {
        let dx = player.position.0 - self.position.0;
        let dy = player.position.1 - self.position.1;
        
        // Calculate the direction to move (one step towards player)
        let move_x = if dx > 0 { 1 } else if dx < 0 { -1 } else { 0 };
        let move_y = if dy > 0 { 1 } else if dy < 0 { -1 } else { 0 };
        
        let new_pos = (self.position.0 + move_x, self.position.1 + move_y);
        
        // Check if we would move onto the player - if so, attack instead
        if new_pos == player.position {
            // Attack the player
            let mut rng = rand::thread_rng();
            let damage = rng.gen_range(5..=20);
            player.take_damage(damage);
            log_messages.push(format!("The orc {} attacks you for {} damage!", self.name, damage));
            return;
        }
        
        // Check if the new position is valid and walkable
        if !world.is_valid_position(new_pos.0, new_pos.1) || !world.is_walkable(new_pos.0, new_pos.1) {
            return; // Can't move there
        }
        
        // Check if another NPC is at the new position
        if other_npcs.iter().any(|npc| npc.position == new_pos) {
            return; // Can't move into another NPC
        }
        
        // Move the orc
        self.position = new_pos;
    }
    
    /// Try to move the orc randomly (for when far from player)
    fn try_random_move_orc(&mut self, world: &mut GameWorld, player: &Player, other_npcs: &[NPC], rng: &mut impl Rng) {
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
            
            // Valid move found - move the orc
            self.position = new_pos;
            return; // Successfully moved, exit the function
        }
        
        // If we get here, no valid move was found after 2 attempts
    }
}