# Engine

A Rust-based game engine built with wgpu for high-performance graphics rendering.

## Project Structure

This is a Cargo workspace containing:

- `engine/` - The core game engine library
- `sandbox/` - Example application demonstrating engine usage

## Features

- High-performance graphics rendering using wgpu
- Cross-platform support (Windows, Linux, macOS)
- Vulkan, DirectX 12, and OpenGL backends
- Entity-component system architecture
- Input handling
- Window management with winit

## Dependencies

- `wgpu` 0.20 - Modern graphics API
- `winit` 0.30 - Window creation and input handling
- `anyhow` - Error handling

## Building

Ensure you have Rust installed (https://rustup.rs/).

```bash
# Clone the repository
git clone <repository-url>
cd engine

# Build the entire workspace
cargo build

# Build in release mode for better performance
cargo build --release
```

## Running the Sandbox

```bash
# Run the example application
cargo run -p sandbox

# Run in release mode
cargo run -p sandbox --release
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a complete history of changes and version releases.

## Troubleshooting

### Graphics Driver Issues

If you encounter `STATUS_ACCESS_VIOLATION` errors:

1. Update your graphics drivers to the latest version
2. Try forcing a specific backend:
   ```bash
   # Force Vulkan
   WGPU_BACKEND=vulkan cargo run -p sandbox
   
   # Force DirectX 12
   WGPU_BACKEND=dx12 cargo run -p sandbox
   
   # Force OpenGL
   WGPU_BACKEND=gl cargo run -p sandbox
   ```

### Common Issues

- **No suitable GPU adapter found**: Ensure your graphics drivers support Vulkan, DX12, or OpenGL
- **Window doesn't appear**: Check that your display settings allow new windows
- **Performance issues**: Build in release mode with `cargo build --release`

## Architecture

See the [docs/architecture/](docs/architecture/) folder for detailed system design and architecture documentation.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]