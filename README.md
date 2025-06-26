# Slack Game - 2D Platformer

A 2D side-scrolling platformer game inspired by Super Mario Bros, built with Rust and macroquad. This project demonstrates game development fundamentals including physics, collision detection, animation, and level progression.

## Features

### Core Gameplay
- **Player Character**: Animated sprite with idle, walking, and jumping states
- **Physics System**: Gravity, jumping, smooth horizontal movement with friction
- **Platform System**: Textured platforms with grass and dirt details
- **Camera System**: Smooth camera following with proper viewport management

### Game Elements
- **Enemies**: Purple Goomba-like creatures with AI patrol behavior
- **Collectibles**: Gold coins with sparkle effects (10 points each)
- **Power-ups**: Speed boost (orange) and jump boost (blue) with glow effects
- **Goal System**: Mario-style flag poles for level completion

### Level Progression
- **3 Complete Levels**:
  - Level 1-1: Tutorial level with basic platforming
  - Level 1-2: Underground level with more enemies and longer gaps
  - Level 1-3: Castle level with challenging precision jumps
- **Progressive Difficulty**: More enemies, smaller platforms, longer levels
- **Level Completion**: 1000 bonus points for reaching the flag

### Game States
- **Start Screen**: Instructions and controls
- **Playing**: Full gameplay with lives system (3 lives)
- **Paused**: Semi-transparent overlay (ESC to pause/resume)
- **Game Over**: Final score display with restart/menu options
- **Victory Screen**: Congratulations for completing all levels

### Visual Features
- **Sprite-based Graphics**: Detailed pixel-art style characters and objects
- **Animation System**: Frame-based animation for player movement
- **Background Graphics**: Sky gradient with cloud sprites
- **UI Elements**: Score, lives, level counter, and control hints

### Audio System
- **Sound Effects**: Jump, coin collection, enemy defeat, power-up, hit sounds
- **Audio Events**: Console-based audio feedback for all game actions

## Controls

- **Movement**: A/D or Left/Right arrow keys
- **Jump**: W/Up arrow/Space bar
- **Pause**: Escape key
- **Menu Navigation**: Space/Enter to start, R to restart, Escape for main menu

## Technical Implementation

### Architecture
- **Entity System**: Separate structs for Player, Enemy, Coin, PowerUp, Platform
- **Level System**: Data-driven level layouts with goal positions
- **Game State Management**: Enum-based state machine for different screens
- **Camera System**: 2D camera with smooth following behavior

### Physics & Collision
- **Gravity System**: 800 pixels/second² downward acceleration
- **Collision Detection**: Rectangle-based AABB collision with separate X/Y axis handling
- **Platform Physics**: Prevents falling through surfaces, supports jumping on enemies
- **Movement Physics**: Velocity-based movement with friction and delta time

### Code Structure
- **Modular Design**: Clean separation of concerns between game systems
- **Rust Best Practices**: Proper ownership, borrowing, and memory safety
- **Performance**: Efficient collision detection and rendering

## Development Journey

This project was built incrementally through 5 phases following a structured game development approach:

### Phase 1: Foundation
- Project setup with Rust and macroquad
- Basic game window and main loop
- Input system implementation

### Phase 2: Player Character
- Player entity creation and rendering
- Movement mechanics with physics
- Gravity and jumping system

### Phase 3: World Building
- Platform system with collision detection
- Improved collision handling (separate X/Y axes)
- Camera system implementation

### Phase 4: Game Mechanics
- Enemy AI and collision system
- Collectibles and power-up system
- Game state management (start, pause, game over)

### Phase 5: Polish & Enhancement
- Audio system integration
- Visual improvements and animations
- Multiple level system with progressive difficulty

## Building and Running

```bash
# Clone the repository
git clone <repository-url>
cd slack_game

# Run the game
cargo run

# Build for release
cargo build --release
```

## Dependencies

- **macroquad**: 2D game framework for Rust
- **Rust Edition**: 2024

## Game Design Inspiration

This game draws inspiration from classic 2D platformers, particularly Super Mario Bros, featuring:
- Side-scrolling gameplay
- Jump-on-enemies mechanics
- Collectible coins and power-ups
- Flag pole level completion
- Progressive level difficulty

## Learning Outcomes

This project demonstrates:
- Game loop architecture and state management
- 2D physics simulation and collision detection
- Animation systems and sprite rendering
- Level design and progression systems
- Audio integration and user experience design
- Rust ownership model in game development context

---

*Built with Rust 🦀 and macroquad 🎮*