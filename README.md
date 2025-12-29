# Blackspire

A classic roguelike blackspire game built with Rust, featuring procedurally generated dungeons, turn-based combat, and an Entity Component System (ECS) architecture. Navigate through dangerous levels, battle monsters, collect items, and retrieve the legendary Amulet of Yala to save your hometown!

## 🎮 Features

- **Procedurally Generated Dungeons**: Three different map generation algorithms (Rooms, Cellular Automata, Drunkard's Walk) create unique dungeons every playthrough
- **Turn-Based Combat**: Strategic turn-based gameplay where every move matters
- **Field of View**: Realistic line-of-sight mechanics - you can only see what your character can see
- **Multiple Enemy Types**: Face off against Goblins, Orcs, Ogres, and Ettins, each with different stats and behaviors
- **Item System**: Collect and use healing potions, weapons, and magical items
- **Multiple Levels**: Progress through multiple dungeon levels, each more challenging than the last
- **Themes**: Different visual themes (Dungeon and Forest) for varied aesthetics
- **HUD**: Comprehensive heads-up display showing health, inventory, and game information
- **Tooltips**: Hover over entities to see their names and details

## 🛠️ Technology Stack

- **Language**: Rust (Edition 2021+)
- **ECS Framework**: [Legion](https://github.com/amethyst/legion) - High-performance Entity Component System
- **Game Engine**: [bracket-lib](https://github.com/amethyst/bracket-lib) - Terminal-based game library
- **Serialization**: [RON](https://github.com/ron-rs/ron) - Rusty Object Notation for template files
- **Serialization Framework**: [Serde](https://serde.rs/) - Framework for serializing and deserializing Rust data structures

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

- **Rust**: Version 1.70.0 or later
  - Install from [rustup.rs](https://rustup.rs/)
  - Verify installation: `rustc --version`
- **Cargo**: Comes bundled with Rust
  - Verify installation: `cargo --version`

### Platform-Specific Requirements

#### Windows
- Visual Studio Build Tools or Visual Studio with C++ development tools
- Windows SDK

#### Linux
- `build-essential` package (includes GCC, make, etc.)
  ```bash
  sudo apt-get update
  sudo apt-get install build-essential
  ```

#### macOS
- Xcode Command Line Tools
  ```bash
  xcode-select --install
  ```

## 🚀 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/blackspire.git
   cd blackspire/blackspire
   ```

2. **Verify the project structure**
   Ensure you have the following structure:
   ```
   blackspire/
   ├── Cargo.toml
   ├── src/
   │   ├── main.rs
   │   └── ...
   └── resources/
       ├── dungeonfont.png
       ├── terminal8x8.png
       └── template.ron
   ```

## 🔨 Building and Compiling

### Debug Build

Build the project in debug mode (faster compilation, slower runtime):

```bash
cargo build
```

The executable will be located at: `target/debug/blackspire.exe` (Windows) or `target/debug/blackspire` (Linux/macOS)

### Release Build

Build an optimized release version (slower compilation, faster runtime):

```bash
cargo build --release
```

The optimized executable will be located at: `target/release/blackspire.exe` (Windows) or `target/release/blackspire` (Linux/macOS)

**Note**: The release build uses Link-Time Optimization (LTO) for maximum performance.

### Check for Compilation Errors

Check if the code compiles without building:

```bash
cargo check
```

## ▶️ Running the Game

### Quick Start

Run the game directly with Cargo (debug mode):

```bash
cargo run
```

Run the optimized release version:

```bash
cargo run --release
```

### Running the Executable Directly

After building, you can run the executable directly:

**Windows:**
```bash
.\target\release\blackspire.exe
```

**Linux/macOS:**
```bash
./target/release/blackspire
```

**Important**: Make sure you run the executable from the `blackspire` directory, as the game expects the `resources/` folder to be in the current working directory.
You can also copy the executable to a different folder but ensure you also copy the resources folder into the folder that contains your executable.

## 🎯 How to Play

### Objective

Your goal is to navigate through multiple dungeon levels, defeat monsters, collect items, and ultimately retrieve the **Amulet of Yala** on the final level to save your hometown.

### Controls

| Key | Action |
|-----|--------|
| `↑` `↓` `←` `→` | Move player (Arrow Keys) |
| `G` | Pick up item at current location |
| `0-9` | Use item from inventory (0-9 correspond to inventory slots) |

### Gameplay Mechanics

- **Turn-Based**: The game uses a turn-based system. After you move, enemies take their turn
- **Combat**: Move into an enemy to attack them. Combat is automatic
- **Health**: Your health is displayed in the HUD. If it reaches 0, it's game over
- **Field of View**: You can only see tiles within your field of view radius
- **Level Progression**: Find the exit staircase (`>`) to advance to the next level
- **Victory**: Collect the Amulet of Yala (`|`) on the final level to win

### Items

- **Healing Potion** (`!`): Restores health when used
- **Dungeon Map** (`{`): Reveals the entire map
- **Rusty Sword** (`/`): Weapon that increases damage
- **Shiny Sword** (`S`): Better weapon with higher damage
- **Huge Sword** (`/`): Powerful weapon for later levels

### Enemies

- **Goblin** (`g`): Weak enemy, 1 HP, appears on all levels
- **Orc** (`o`): Moderate enemy, 2 HP, appears on all levels
- **Ogre** (`O`): Strong enemy, 5 HP, appears on levels 1-2
- **Ettin** (`E`): Very strong enemy, 10 HP, appears on level 2

## 📁 Project Structure

```
dungeon-dragons/
├── Cargo.toml              # Project metadata and dependencies
├── Cargo.lock              # Locked dependency versions
├── README.md               # This file
├── resources/              # Game assets
│   ├── dungeonfont.png    # Main game font
│   ├── terminal8x8.png    # Terminal font
│   └── template.ron       # Entity templates (monsters, items)
├── src/
│   ├── main.rs            # Entry point and game loop
│   ├── camera.rs          # Camera system for viewport
│   ├── components.rs      # ECS component definitions
│   ├── map.rs             # Map and tile definitions
│   ├── player.rs          # Player-related logic
│   ├── turn_state.rs      # Game state machine
│   ├── map_builder/       # Procedural map generation
│   │   ├── mod.rs
│   │   ├── automata.rs    # Cellular automata algorithm
│   │   ├── drunkard.rs    # Drunkard's walk algorithm
│   │   ├── rooms.rs       # Room-based algorithm
│   │   ├── prefab.rs      # Prefab room placement
│   │   └── themes.rs      # Visual themes
│   ├── spawner/           # Entity spawning logic
│   │   ├── mod.rs
│   │   └── template.rs    # Template loading and spawning
│   └── systems/           # ECS systems
│       ├── mod.rs
│       ├── chasing.rs     # Enemy AI - chasing player
│       ├── combat.rs      # Combat resolution
│       ├── end_turn.rs    # Turn state transitions
│       ├── entity_render.rs # Entity rendering
│       ├── fov.rs         # Field of view calculations
│       ├── hud.rs         # Heads-up display
│       ├── map_render.rs  # Map rendering
│       ├── movement.rs    # Movement handling
│       ├── player_input.rs # Player input handling
│       ├── random_move.rs # Enemy AI - random movement
│       ├── tooltip.rs     # Tooltip system
│       └── use_item.rs    # Item usage system
└── target/                # Build output (gitignored)
```

## 🐛 Troubleshooting

### Common Issues

**Issue**: Game crashes with "Couldn't find specified file" error
- **Solution**: Make sure you're running the game from the `dungeon-dragons` directory where the `resources/` folder is located

**Issue**: Build fails with linker errors
- **Solution**: Ensure you have the required build tools installed for your platform (see Prerequisites)

**Issue**: Game runs slowly
- **Solution**: Use the release build (`cargo run --release`) instead of debug build

**Issue**: Fonts not displaying correctly
- **Solution**: Verify that `resources/dungeonfont.png` and `resources/terminal8x8.png` exist in the correct location

## 🤝 Contributing

Contributions are welcome! This project follows open-source best practices and we appreciate your help in making it better.

### How to Contribute

1. **Fork the repository**
   - Click the "Fork" button on GitHub

2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```
   Or for bug fixes:
   ```bash
   git checkout -b fix/your-bug-fix
   ```

3. **Make your changes**
   - Write clean, well-commented code
   - Follow Rust conventions and style guidelines
   - Add comments for complex logic
   - Update documentation as needed

4. **Test your changes**
   ```bash
   cargo build --release
   cargo run --release
   ```
   Ensure the game compiles and runs correctly

5. **Commit your changes**
   ```bash
   git add .
   git commit -m "Description of your changes"
   ```
   Write clear, descriptive commit messages

6. **Push to your fork**
   ```bash
   git push origin feature/your-feature-name
   ```

7. **Create a Pull Request**
   - Go to the original repository on GitHub
   - Click "New Pull Request"
   - Select your fork and branch
   - Fill out the PR template with:
     - Description of changes
     - What was changed and why
     - Any breaking changes
     - Screenshots (if UI changes)

### Contribution Guidelines

- **Code Style**: Follow Rust's official style guide. Use `cargo fmt` to format your code
- **Documentation**: Add doc comments for public functions and types
- **Testing**: Add tests for new features when possible
- **Breaking Changes**: Clearly document any breaking changes in your PR
- **Issues**: Before starting work on a large feature, consider opening an issue to discuss it first

### Areas for Contribution

- 🐛 Bug fixes
- ✨ New features (new enemies, items, map generation algorithms)
- 📚 Documentation improvements
- 🎨 UI/UX enhancements
- ⚡ Performance optimizations
- 🧪 Additional tests
- 🌍 Localization support

## 📝 Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow:

- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Respect different viewpoints and experiences

## 📄 License

This project is currently unlicensed. If you'd like to use this code, please contact the author or consider adding a license file.

## 🙏 Acknowledgments

- Built with [bracket-lib](https://github.com/amethyst/bracket-lib) by The Amethyst Foundation
- Uses [Legion ECS](https://github.com/amethyst/legion) for entity management
- Inspired by classic roguelike games like Rogue, NetHack, and Angband

## 📧 Contact

**Author**: James Muriuki Maina  
**Email**: geniusinrust@gmail.com

## 🔮 Future Plans

Potential features for future versions:

- [ ] Save/Load game functionality
- [ ] More enemy types and behaviors
- [ ] Additional items and equipment
- [ ] Character classes with unique abilities
- [ ] More map generation algorithms
- [ ] Sound effects and music
- [ ] Configurable keybindings
- [ ] Difficulty levels
- [ ] High score system
- [ ] Multiplayer support

## 📊 Project Status

**Current Version**: 0.1.0 (MVP)

This project is currently in MVP (Minimum Viable Product) status. The core gameplay loop is complete and functional, but there's plenty of room for expansion and improvement.

---

**Enjoy your dungeon crawling adventure!** 🗡️🛡️⚔️

