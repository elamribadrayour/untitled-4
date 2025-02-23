# Grid Animation

A creative coding project built with Rust and Nannou that generates an animated grid with customizable color palettes.

<div align="center">
    <video width="600" controls>
    <source src="result.mp4" type="video/mp4">
    Your browser does not support the video tag.
    </video>
</div>

## Features

- 10x10 grid of interconnected points
- Random subtle movement animation ("za3za3" effect)
- Two color palette options:
  - Solarized theme
  - Game theme
- Automatic frame capture for animation export

## Prerequisites

- Rust (2021 edition)
- Cargo

## Dependencies

- nannou (0.19.0) - Creative coding framework
- nannou_wgpu (0.19.0) - GPU backend
- tokio (1.43.0) - Async runtime
- futures (0.3.31) - Async utilities
- anyhow (1.0.96) - Error handling

## Installation

1. Clone the repository
2. Run with Cargo:

```bash
cargo run
```

## Configuration

The project includes several configurable constants in `src/env.rs`:

- Window dimensions: 800x800 pixels
- Grid size: 10x10
- Padding: 50.0 pixels

## Color Palettes

### Solarized Theme
- Background: Deep blue (#073642)
- Cycling colors:
  - Magenta (#D33682)
  - Cyan (#2AA198)
  - Purple (#6C71C4)

### Game Theme
- Background: Light cream (#F6EEE3)
- Cycling colors:
  - Red (#D63230)
  - Yellow (#F7D002)
  - Blue (#1A53C0)
  - Navy (#0D274B)

## Output

The application automatically captures frames in the `output` directory, which can be used to create animations or review the generated patterns.

## License

This project is licensed under the WTFPL License [LICENSE](LICENSE)
