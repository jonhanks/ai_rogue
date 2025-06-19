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
        }
    }

    pub fn display_info(&self) -> (char, (u8, u8, u8)) {
        let char = self.get_display_char();
        let color = match self.item_type {
            ItemType::Key => (255, 215, 0), // Gold
            ItemType::TreasureChest => (139, 69, 19), // Brown
            ItemType::Treasure => (255, 215, 0), // Gold
        };
        (char, color)
    }
}

