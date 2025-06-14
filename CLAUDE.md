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

The codebase follows a layered architecture with clear separation of concerns:

- **Game Data**: Core game structures (`Player`, `GameWorld`, `TileType`) handle game state and logic
- **UI Layer**: `RoguelikeApp` implements the eframe::App trait and manages the GUI
- **Game State**: `GameState` coordinates between player, world, and UI components

### Key Components

- `Player`: Manages player stats, position, inventory, and actions (health, movement, etc.)
- `GameWorld`: Handles the dungeon layout using a 2D tile grid system with different tile types
- `GameState`: Central coordinator that manages player-world interactions and game log
- `RoguelikeApp`: egui-based UI that renders the world view and information panels

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