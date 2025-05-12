# Snake (Rust + SDL2)

A clean Snake clone made with Rust and SDL2.

## Features

- Classic snake mechanics
- Score system
- Keyboard controls

## Requirements

- **Rust** (stable) — install via [rustup](https://rustup.rs)
- **SDL2** — graphics and input
- **SDL2_ttf** — for rendering text (e.g., score display)

### SDL2 and SDL2_ttf installation

#### Linux
```bash
sudo apt install llibsdl2-dev libsdl2-ttf-dev
```

#### macOS (using Homebrew)
```bash
brew install sdl2 sdl2_ttf
```

#### Windows
Use [vcpkg](https://github.com/microsoft/vcpkg) or manually install SDL2 and SDL2_ttf developer packages.
Make sure the .ddl files are in your `PATH` or project folder at runtime.

## Building the Project
Clone the repository and build in release mode:
```bash
git clone https://github.come/{io}/rusty-tetris
cd rusty-tetris
cargo build --release
```

## Running the game
```bash
cargo run --release
```

## Controls
- Arrow keys / WASD to move the snake

## Development Notes
This project uses the following crates:
- sdl2
- rand

TO regenerate documentation locally:
```bash
cargo doc --open
```

## License
MIT License. See [LICENSE](LICENSE) for more details.

