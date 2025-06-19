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

### Completed Refactoring (Latest Session)
1. **Module Restructuring**: Split code into separate modules (item.rs, npc.rs, game_condition.rs)
2. **Game Condition System**: Implemented trait-based win/loss conditions for flexible game types
3. **Constructor Usage**: Updated to use proper constructors instead of Default trait
4. **Warning Cleanup**: Reduced compiler warnings from 13 to 7 by fixing unused imports and using constructors

### Current Architecture State
- ✅ Modular code organization with clear separation of concerns
- ✅ Trait-based game condition system for extensible game rules
- ✅ Clean module boundaries between UI, game logic, and data structures
- ✅ Dynamic goal text based on active game condition

### Remaining Warnings (Non-critical)
- Some unused methods/variants that are intended for future features
- These provide useful functionality for game expansion

### Next Potential Improvements
- Implement additional game condition types
- Add turn-based mechanics for survival conditions  
- Expand NPC interaction system
- Add more item types and effects