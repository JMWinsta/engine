# Engine Architecture

## Overview

The Engine is a Rust-based game engine designed for high-performance 2D/3D graphics applications. It uses wgpu for cross-platform graphics rendering and follows an entity-component-system (ECS) architecture pattern.

## Core Components

### Renderer System

The renderer system handles all graphics operations using wgpu:

- **wgpu Instance**: Manages graphics backends (Vulkan, DX12, OpenGL)
- **Adapter**: Represents the physical GPU
- **Device & Queue**: Low-level GPU interface for command submission
- **Surface**: Window surface for rendering
- **Render Pipeline**: Configured pipeline for drawing operations
- **Shaders**: WGSL shaders for vertex and fragment processing

### Entity System

Entities are the basic building blocks of game objects:

- **Entity**: Unique identifier for game objects
- **Components**: Data attached to entities (position, velocity, renderable, etc.)
- **Systems**: Logic that operates on entities with specific components

### Physics System

The physics system implements a custom 2D rigid body dynamics engine:

- **Broadphase**: Uniform Grid spatial partitioning to quickly find collision candidates.
- **Narrowphase**: Separating Axis Theorem (SAT) for precise collision manifolds (circle, AABB, polygon).
- **Solver**: Sequential impulse resolution handling positional error correction, bouncing, and Coulomb friction.
- **Integrator**: Semi-implicit Euler integration, applying gravity, dampening, and tracking sleep states.

### Input System

Handles user input from keyboard, mouse, and other devices:

- **Keyboard**: Key press/release events
- **Mouse**: Position, button states, wheel events
- **Window Events**: Resize, focus, close events

### Window System

Manages the application window using winit:

- **Window Creation**: Cross-platform window management
- **Event Loop**: Main application loop processing events
- **Surface Management**: Integration with wgpu surface

## Data Flow

```
Input Events → Input System → Entity Updates
                    ↓
Entity Updates → Physics System → Integrator → Resolved Collision State
                    ↓
Window Events → Window System → Surface Updates
                    ↓
Entity Data → Renderer System → GPU Commands → Display
```

## Future Extensions

- 3D Physics and Rendering upgrade
- Audio system
- Networking capabilities
- Asset loading and management
- Scene management
- UI framework

## Performance Considerations

- GPU-driven rendering for optimal performance
- Efficient memory management with wgpu
- Asynchronous command submission
- Frame rate independent updates

## Diagrams

See the [diagrams/](diagrams/) folder for visual architecture diagrams.

_Last updated: 2026-04-05_
