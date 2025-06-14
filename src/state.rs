#[derive(Debug, Clone)]
pub struct Player {
    pub position: (i32, i32),
    pub health: i32,
    pub max_health: i32,
    pub level: i32,
    pub experience: i32,
    pub inventory: Vec<Item>,
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
    pub items: Vec<WorldItem>, // Items placed in the world
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    Door,
    Stairs,
    Empty,
}

#[derive(Debug, Clone)]
pub struct NPC {
    pub position: (i32, i32),
    pub inventory: Vec<Item>,
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

#[derive(Debug, Clone)]
pub struct Item {
    pub item_type: ItemType,
    pub label: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Weapon,
    Armor,
    Potion,
    Food,
    Tool,
    Key,
    Treasure,
}

impl Item {
    pub fn new(item_type: ItemType, label: String, description: String) -> Self {
        Self {
            item_type,
            label,
            description,
        }
    }

    pub fn get_display_char(&self) -> char {
        match self.item_type {
            ItemType::Weapon => '/',
            ItemType::Armor => '[',
            ItemType::Potion => '!',
            ItemType::Food => '%',
            ItemType::Tool => '(',
            ItemType::Key => '-',
            ItemType::Treasure => '$',
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorldItem {
    pub position: (i32, i32),
    pub item: Item,
}

impl WorldItem {
    pub fn new(x: i32, y: i32, item: Item) -> Self {
        Self {
            position: (x, y),
            item,
        }
    }
}

impl NPC {
    pub fn new(x: i32, y: i32, npc_type: NPCType, name: String) -> Self {
        Self {
            position: (x, y),
            inventory: Vec::new(),
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

    pub fn move_to(&mut self, new_pos: (i32, i32)) {
        self.position = new_pos;
    }
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
            items: Vec::new(),
        }
    }
}

impl GameWorld {
    pub fn new(width: usize, height: usize) -> Self {
        let mut world = Self {
            size: (width, height),
            current_floor: 1,
            tiles: vec![vec![TileType::Empty; height]; width],
            items: Vec::new(),
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
    pub npcs: Vec<NPC>,
    pub log_messages: Vec<String>,
    pub game_over: bool,
}

impl GameState {
    pub fn new() -> Self {
        let mut npcs = Vec::new();
        npcs.push(NPC::new(5, 5, NPCType::Goblin, "Grob".to_string()));
        npcs.push(NPC::new(15, 8, NPCType::Merchant, "The Merchant".to_string()));
        npcs.push(NPC::new(25, 12, NPCType::Skeleton, "Bonecrusher".to_string()));
        npcs.push(NPC::new(8, 20, NPCType::Guard, "Guard Captain".to_string()));

        Self {
            player: Player::default(),
            world: GameWorld::default(),
            npcs,
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

        if !self.world.is_valid_position(new_pos.0, new_pos.1) ||
            !self.world.is_walkable(new_pos.0, new_pos.1) {
            self.add_log_message("Can't move there!".to_string());
            return false;
        }

        // Check for NPC collision
        if let Some(npc_index) = self.npcs.iter().position(|npc| npc.position == new_pos) {
            // Remove NPC temporarily to avoid borrow checker issues
            let mut npc = self.npcs.remove(npc_index);
            
            // Interact with NPC instead of moving
            self.interact_with_npc(&mut npc);
            
            // Add NPC back to the vector
            self.npcs.push(npc);
            false
        } else {
            // Move player
            self.player.move_to(new_pos);
            self.add_log_message(format!("Moved to ({}, {})", new_pos.0, new_pos.1));
            true
        }
    }

    pub fn interact_with_npc(&mut self, npc: &mut NPC) {
        self.add_log_message(format!("You interact with {}.", npc.name));
    }
}