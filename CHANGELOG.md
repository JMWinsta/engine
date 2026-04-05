# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-04-05

### Added

- Core 2D Physics Engine featuring rigid body dynamics.
- Advanced Collision Detection (Narrowphase) utilizing Seperating Axis Theorem (SAT).
- AABB uniform grid broadphase for optimized collision queries.
- Impulse-based velocity and friction physics solver with positional correction.
- Implemented semi-implicit Euler integration for reliable state progression.
- Testbed sandbox setups including: Stacking scenes, distance-constrained Pendulum, collision Momentum Transfer, and a WASD interactive Platformer test.
- CI/CD workflow for automated testing and building
- Comprehensive documentation structure
- Architecture and system design documentation

### Changed

- Updated wgpu to version 0.20 for better compatibility
- Improved graphics adapter selection with fallback logic
- Extracted and smoothed render interpolation independent of fixed-tick physics loops.

### Fixed

- Resolved STATUS_ACCESS_VIOLATION crash on Windows systems
- Added proper graphics backend detection and fallback

## [0.1.0] - 2026-03-31

### Added

- Initial Rust game engine implementation
- Core engine library with entity system, input handling, and rendering
- Sandbox application for testing engine features
- Basic physics simulation with entity movement
- WGPU-based graphics rendering with shader support
- Window management using winit
- Modular architecture with separate crates (engine, sandbox)

### Technical Details

- **Graphics**: WGPU with Vulkan/OpenGL/DX12 backends
- **Windowing**: Winit for cross-platform window management
- **Language**: Rust with Cargo workspace structure
- **Architecture**: Entity-Component-System pattern

[Unreleased]: https://github.com/JMWinsta/engine/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/JMWinsta/engine/releases/tag/v0.1.0
