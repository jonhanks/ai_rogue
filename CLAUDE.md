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
  - Survival Challenge: Survive for 50 turns without dying
  - Item Collection: Collect 3 gems, 2 scrolls, and 1 potion
- **New Item Types**: Added Gem (*), Scroll (?), and Potion (!) with unique colors
- **Dynamic Goals**: Goal text updates based on selected game type

### Display System Improvements
- **Cleaner Function Signatures**: Display functions now take `&GameState` directly
- **Reduced Code Duplication**: Eliminated repeated `Option<GameState>` checks
- **WorldViewInteraction Structure**: Proper return type for world interactions
- **Better Separation**: UI rendering separated from state management
- **Immutable Display**: Display functions are now read-only operations

### Current Architecture State
- ✅ Modular code organization with clear separation of concerns
- ✅ Trait-based game condition system for extensible game rules
- ✅ Game type selection UI for varied gameplay experiences
- ✅ Clean display function architecture with structured interactions
- ✅ Extended item system supporting diverse game modes
- ✅ Dynamic goal text based on active game condition

### Code Quality Improvements
- **Reduced Warnings**: From 13 to 5 compiler warnings
- **Better Readability**: Less nested code, clearer function purposes
- **Maintainability**: Easier to modify display logic without state concerns
- **Extensibility**: Ready for new interaction types and game modes

### Remaining Warnings (Non-critical)
- Some unused methods/variants that are intended for future features
- These provide useful functionality for game expansion

### Next Potential Improvements
- Create different map generators for different game types
- Add turn-based mechanics for survival conditions
- Implement click handling through WorldViewInteraction structure
- Expand NPC interaction system with new dialog types
- Add context menus and multi-selection capabilities