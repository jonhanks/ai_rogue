#[derive(Debug, Clone)]
pub struct Item {
    pub item_type: ItemType,
    pub label: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Key,
    TreasureChest,
    Treasure,
    Gem,
    Scroll,
    Potion,
}

#[derive(Debug)]
pub struct ItemUseResult {
    pub returned_to_inventory: Option<Item>,
    pub dropped_on_ground: Vec<Item>,
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
            ItemType::Key => '-',
            ItemType::TreasureChest => '=',
            ItemType::Treasure => '$',
            ItemType::Gem => '*',
            ItemType::Scroll => '?',
            ItemType::Potion => '!',
        }
    }

    pub fn display_info(&self) -> (char, (u8, u8, u8)) {
        let char = self.get_display_char();
        let color = match self.item_type {
            ItemType::Key => (255, 215, 0), // Gold
            ItemType::TreasureChest => (139, 69, 19), // Brown
            ItemType::Treasure => (255, 215, 0), // Gold
            ItemType::Gem => (255, 20, 147), // Deep pink
            ItemType::Scroll => (245, 245, 220), // Beige
            ItemType::Potion => (138, 43, 226), // Blue violet
        };
        (char, color)
    }
}

