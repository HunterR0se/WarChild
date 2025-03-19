# War Child

A gravity-based, 2D side-scrolling platform game written in Rust with the [Macroquad](https://github.com/not-fl3/macroquad) game engine.

## Description

War Child is a fast-paced platformer featuring:
- Dynamic gravity-based movement mechanics
- Combat with melee and ranged attacks
- Various enemy types with different attack patterns
- Collectible items and artifacts that enhance player abilities
- Fluid animations and visual effects

## Errata

The current state of the code and game is "In Progress..." and the code is functional, but not even remotely optimized or complete. This is a labour of love by one person and could take months (or longer) to achieve a fully working game without issues.  That said, even with the bounding boxes and in the current state (March 2025), the game is functional enough for it to be somewhat entertaining and fun.

## Requirements

- Rust 1.81.0 or newer
- Cargo package manager
- OpenGL 3.2 compatible graphics hardware

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/username/warchild.git
   cd warchild
   ```

2. Build and run in debug mode:
   ```bash
   cargo run
   ```

3. For better performance, build in release mode:
   ```bash
   cargo run --release
   ```

## Controls

- **Arrow Keys**: Movement (Left/Right) and climbing (Up/Down)
- **Up Arrow**: Up Arrow (double-tap for double jump)
- **Space**: Melee Attack
- **Right Arrow**: Ranged Attack
- **ESC**: Pause Game
- **Q**: Quit to Menu (when paused)

## Building for Different Platforms

### Windows

```bash
cargo build --release
```

### macOS

```bash
cargo build --release
```

### Linux

```bash
cargo build --release
```

### Web (WebAssembly)

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

## Assets

The game includes various assets for graphics and audio. These are required and, while they add to the size of the download, the overall size of the project is still quite small for a functional platformer or game.
