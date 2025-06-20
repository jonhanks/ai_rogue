# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based roguelike game built with egui/eframe for the GUI. The game features a player character (@) that can move around a dungeon world represented by ASCII characters.

## Key Commands

- **Build**: `cargo build`
- **Run**: `cargo run`
- **Check for errors**: `cargo check`
- **Run tests**: `cargo test`

## Architecture

The codebase follows a modular architecture with clear separation of concerns:

### Module Structure
- **`main.rs`**: UI layer with `RoguelikeApp` implementing eframe::App trait
- **`state.rs`**: Core game state with `Player`, `GameWorld`, `GameState`, and `WorldItem`
- **`item.rs`**: Item system with `Item`, `ItemType`, and `ItemUseResult`
- **`npc.rs`**: NPC system with `NPC`, `NPCType`, and `InteractionResult`
- **`game_condition.rs`**: Win/loss condition system with trait-based game rules

### Key Components

- **`Player`**: Manages player stats, position, inventory, and actions (health, movement, etc.)
- **`GameWorld`**: Handles the dungeon layout using a 2D tile grid system with different tile types
- **`GameState`**: Central coordinator that manages player-world interactions, game log, and win/loss conditions
- **`GameCondition` trait**: Flexible system for defining different game types (treasure hunt, survival, collection)
- **`RoguelikeApp`**: egui-based UI that renders the world view and information panels

### Game Condition System

The game uses a trait-based system for win/loss conditions:
- `TreasureHuntCondition`: Default game (collect treasure, don't die)
- `SurvivalCondition`: Survive for X turns
- `CollectionCondition`: Collect specific items
- Easily extensible for new game types

### UI Layout

The interface uses a horizontal split:
- Left panel (75% width): World view with scrollable ASCII representation
- Right panel (25% width): Player stats, inventory, message log, and controls

### Input Handling

Movement is handled via keyboard input (Arrow keys or WASD) processed in the main update loop.

## Development Notes

- All game data structures are separate from UI code for maintainability
- The world uses a coordinate system where (0,0) is top-left
- Player movement validation occurs through `GameWorld::is_walkable()` and `is_valid_position()`
- Message logging is limited to 50 entries to prevent memory issues

## Recent Changes & Current Status

### Completed Refactoring (Latest Sessions)
1. **Module Restructuring**: Split code into separate modules (item.rs, npc.rs, game_condition.rs)
2. **Game Condition System**: Implemented trait-based win/loss conditions for flexible game types
3. **Constructor Usage**: Updated to use proper constructors instead of Default trait
4. **Game Type Selection UI**: Added startup dialog for choosing game mode
5. **Display Function Refactoring**: Cleaned up UI code with proper parameter passing

### Game Type Selection System
- **Startup Experience**: Modal dialog presents game type choices on launch
- **Available Modes**: 
  - Treasure Hunt: Find and collect treasure (classic mode)
  - Survival Challenge: Survive for 200 turns while being hunted by aggressive orcs
  - Item Collection: Collect 3 gems, 2 scrolls, and 1 potion from merchant drops
- **New Item Types**: Added Gem (*), Scroll (?), and Potion (!) with unique colors
- **Dynamic Goals**: Goal text updates based on selected game type with specific requirements

### Display System Improvements
- **Cleaner Function Signatures**: Display functions now take `&GameState` directly
- **Reduced Code Duplication**: Eliminated repeated `Option<GameState>` checks
- **WorldViewInteraction Structure**: Proper return type for world interactions
- **Better Separation**: UI rendering separated from state management
- **Immutable Display**: Display functions are now read-only operations

### NPC AI System
- **Merchant Behavior**: Random movement (24% chance per turn) with item dropping (15% chance) and destruction
- **Orc AI**: Aggressive behavior - hunts player when within 5 spaces, attacks for 5-20 damage
- **Turn-Based Actions**: All NPCs act after player actions in proper turn sequence
- **Collision Detection**: NPCs avoid each other, player, and invalid terrain
- **Dynamic Interactions**: NPCs can modify world state, attack player, and drop/destroy items

### Game Mode-Specific World Generation
- **Treasure Hunt**: Standard mixed NPC setup with treasure chest placement
- **Survival Mode**: 5 aggressive orcs at random positions with random terrain obstacles (15-30 walls)
- **Collection Mode**: Merchant + supporting NPCs for dynamic item collection gameplay
- **Random Positioning**: Survival mode randomizes both player and NPC spawn locations each game

### Turn Counter & Victory Conditions
- **Turn Tracking**: Proper turn counter system tracks all player actions
- **Game-Specific Victory Messages**: Each mode displays appropriate victory text
- **UI Integration**: Turn counter displayed in player stats panel
- **Accurate Survival**: 200-turn survival challenge with proper turn counting

### Current Architecture State
- ✅ Modular code organization with clear separation of concerns
- ✅ Trait-based game condition system for extensible game rules
- ✅ Game type selection UI for varied gameplay experiences
- ✅ Clean display function architecture with structured interactions
- ✅ Extended item system supporting diverse game modes
- ✅ Dynamic goal text based on active game condition
- ✅ Advanced NPC AI system with merchant and orc behaviors
- ✅ Game mode-specific world generation with random elements
- ✅ Turn-based gameplay with proper action sequencing
- ✅ Random terrain generation for survival mode variety

### Code Quality Improvements
- **Reduced Warnings**: From 13 to 5 compiler warnings
- **Better Readability**: Less nested code, clearer function purposes
- **Maintainability**: Easier to modify display logic without state concerns
- **Extensibility**: Ready for new interaction types and game modes

### Remaining Warnings (Non-critical)
- Some unused methods/variants that are intended for future features
- These provide useful functionality for game expansion

### Recent Major Updates (Latest Sessions)
1. **Game Mode-Specific Victory Messages**: Each game type now shows appropriate victory text
2. **Advanced NPC AI System**: Implemented merchant and orc behaviors with complex movement patterns
3. **Turn Counter System**: Added proper turn tracking for accurate gameplay mechanics
4. **Random World Generation**: Survival mode features random terrain and spawn positions
5. **Enhanced Survival Challenge**: Increased to 200 turns with aggressive orc AI pursuit

### Game Balance & Mechanics
- **Survival Difficulty**: 200-turn challenge with 5 aggressive orcs and random terrain
- **Item Economy**: Merchant drops items (15% chance) but also destroys ground items
- **Combat System**: Orcs deal 5-20 damage and actively hunt players within 5-space radius
- **Random Elements**: Each survival game has unique terrain layout and spawn positions

### Next Potential Improvements
- Add more complex map generation algorithms for different biomes
- Implement additional NPC types with unique behaviors
- Add item crafting or upgrading systems
- Expand combat mechanics with weapons and armor
- Create multi-floor dungeon progression
- Add sound effects and visual polish
- Implement save/load functionality