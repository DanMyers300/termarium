# Rust Terminal Aquarium

Welcome to **Termarium**! This is a terminal-based aquarium simulation built in Rust. Watch fish swim around and bubbles rise to the surface, all rendered dynamically in your terminal window.

---

## Features

- 🐟 **Fish Animation**: Fish swim around the terminal, bouncing off the edges dynamically.
- 💨 **Bubble Simulation**: Bubbles rise to the top of the aquarium, with random sizes and movement.
- 🖥️ **Terminal-Based**: Uses ANSI escape codes to render graphics directly in the terminal.
- ⏱️ **Real-Time Updates**: The aquarium updates every 500 milliseconds for smooth animations.
- 🚀 **Cross-Platform**: Works on any terminal that supports ANSI escape codes (Linux, macOS, Windows with proper terminal emulators).
- 🧑‍💻 **Nix Flake Support**: Reproducible builds and a development shell for contributors.

---

## Demo

Here’s what the aquarium looks like in action:

```
  _                     ∘˙○˚.• 
><_>                   .•       
                        ˚○       
  _                                
><_>                              
```

Fish swim around, and bubbles rise to the top!

---

## Installation

### Using Nix Flakes

If you use Nix, you can build and run the project easily with the provided flake:

1. **Run the Aquarium**:
   ```bash
   nix run
   ```

2. **Enter the Development Shell**:
   ```bash
   nix develop
   ```
   This will provide a shell with `cargo`, `rustc`, and `clippy` pre-installed for development.

### Without Nix

1. **Install Rust**:
   Ensure you have Rust installed. You can install it using [rustup](https://rustup.rs/).

2. **Clone the Repository**:
   ```bash
   git clone https://github.com/danmyers300/termarium.git
   cd termarium
   ```

3. **Run the Aquarium**:
   ```bash
   cargo run
   ```

---

## How It Works

### Main Components

1. **Fish**:
   - Fish are represented as ASCII art (`><_>`).
   - They move horizontally and vertically, bouncing off the terminal edges.

2. **Bubbles**:
   - Bubbles are randomly generated and rise to the top of the terminal.
   - They come in two sizes: small and large.

3. **Aquarium**:
   - The `Aquarium` struct manages the fish and bubbles, rendering them dynamically based on the terminal size.

4. **Terminal Interaction**:
   - ANSI escape codes are used to clear the screen, move the cursor, and hide/show the cursor for a seamless experience.

### Code Structure

- `main.rs`: Entry point of the application. Sets up the terminal and runs the simulation loop.
- `aquarium.rs`: Manages the fish and bubbles, rendering them in the terminal.
- `aquarium/fish.rs`: Handles fish movement and rendering.
- `aquarium/bubble.rs`: Handles bubble movement and rendering.

---

## Controls

- **Exit**: Press `Ctrl+C` to exit the aquarium. The cursor will be restored automatically.

---

## Known Issues

- The simulation may not render correctly in terminals that do not support ANSI escape codes.
- Rapid resizing of the terminal may cause temporary glitches.

---

## Contributing

Contributions are welcome! If you have ideas for new features or improvements, feel free to open an issue or submit a pull request.

### Development Setup with Nix

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rust-terminal-aquarium.git
   cd rust-terminal-aquarium
   ```

2. Enter the development shell:
   ```bash
   nix develop
   ```

3. Run the project:
   ```bash
   cargo run
   ```

---

## License

This project is licensed under the [MIT License](LICENSE).

---

Enjoy your aquarium! 🐟💨
