# Rust TUI Chess

A terminal-based chess game written in Rust using [Pleco](https://github.com/pleco-rs/Pleco) chess engine and [Ratatui](https://github.com/ratatui-org/ratatui) for the terminal interface.

## Features

- Full chess implementation with all standard rules
- Terminal-based user interface with Unicode chess pieces
- Cursor-based piece selection and movement
- Move validation using Pleco chess engine
- Simple AI opponent that makes random legal moves
- Game state indicators (check, checkmate, stalemate)
- Move history and status messages
- Keyboard controls

## Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs/))

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/rust-tui-chess
cd rust-tui-chess
```

2. Build and run:
```bash
cargo run
```

## Controls

- Arrow keys: Move cursor
- Enter: Select piece/Make move
- Esc: Deselect piece
- q: Quit game

## Game Rules

- White plays first
- All standard chess rules are implemented
- After each player move, the AI opponent will make a move
- Invalid moves are prevented and reported in the status area
- Game ends on checkmate or stalemate

## Project Structure

```
src/
├── main.rs          # Main game logic and TUI implementation
└── lib.rs           # Library functions (if any)

Cargo.toml           # Project dependencies and metadata
README.md           # This file
```

## Dependencies

- `pleco = "0.5.0"` - Chess move generation and validation
- `ratatui = "0.24.0"` - Terminal user interface
- `crossterm = "0.27.0"` - Terminal manipulation
- `rand = "0.8"` - Random move generation for AI
- `anyhow = "1.0"` - Error handling

## Future Improvements

- [ ] Implement more sophisticated AI using minimax with alpha-beta pruning
- [ ] Add move history display
- [ ] Save/Load game functionality
- [ ] Add clock/timer support
- [ ] Network play support
- [ ] Opening book integration
- [ ] Position evaluation display
- [ ] Multiple AI difficulty levels
- [ ] PGN export functionality

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Pleco Chess Engine](https://github.com/pleco-rs/Pleco)
- [Ratatui](https://github.com/ratatui-org/ratatui)
- Stockfish Chess Engine (which Pleco is derived from)

## Contact

If you have any questions, feel free to reach out by:
- Opening an issue
- Submitting a pull request
- Emailing at [your.email@example.com]