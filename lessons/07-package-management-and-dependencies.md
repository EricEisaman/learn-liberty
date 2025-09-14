# Lesson 7: Package Management and Dependencies

## Learning Objectives
- Understand Cargo vs Python's pip and virtual environments
- Learn about Rust's dependency management system
- Explore the project's dependency structure
- Understand how Rust handles versioning and features

## Rust Idiom of the Day: Cargo vs Python Package Management

In Python, you use pip and virtual environments:
```python
# requirements.txt
numpy==1.21.0
pandas==1.3.0
matplotlib==3.4.0

# Install dependencies
pip install -r requirements.txt

# Virtual environment
python -m venv venv
source venv/bin/activate  # Linux/Mac
# or
venv\Scripts\activate     # Windows
```

In Rust, you use Cargo:
```toml
# Cargo.toml
[dependencies]
wgpu = { version = "0.20", features = ["dx12", "metal"] }
winit = "0.29"
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
env_logger = "0.10"
toml = "0.8"

[dev-dependencies]
tokio-test = "0.4"
mockall = "0.12"
criterion = { version = "0.5", features = ["html_reports"] }
```

**Key Difference**: Cargo manages dependencies at the project level, not globally.

## Game Structural Concept of the Day: Dependency Architecture

Our project's dependency structure:

```
learn-liberty-app
├── Core Dependencies
│   ├── wgpu (Graphics API)
│   ├── winit (Window Management)
│   └── tokio (Async Runtime)
├── Serialization
│   ├── serde (Serialization Framework)
│   ├── serde_json (JSON Support)
│   └── toml (TOML Support)
├── Error Handling
│   └── anyhow (Error Handling)
├── Logging
│   └── env_logger (Logging)
└── Development Dependencies
    ├── tokio-test (Async Testing)
    ├── mockall (Mock Objects)
    └── criterion (Benchmarking)
```

This is similar to Python's dependency structure:
```
learn-liberty-app
├── Core Dependencies
│   ├── pygame (Graphics)
│   ├── tkinter (Window Management)
│   └── asyncio (Async Runtime)
├── Serialization
│   ├── json (JSON Support)
│   └── toml (TOML Support)
├── Error Handling
│   └── built-in exceptions
├── Logging
│   └── logging (Logging)
└── Development Dependencies
    ├── pytest (Testing)
    ├── unittest.mock (Mock Objects)
    └── timeit (Benchmarking)
```

**Key Difference**: Rust's dependency management is more explicit and type-safe.

## Understanding Cargo.toml

The `Cargo.toml` file is the heart of Rust project configuration:

```toml
[package]
name = "learn-liberty-app"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "An educational RPG game"
license = "MIT"
repository = "https://github.com/yourusername/learn-liberty"

[dependencies]
# Version constraints
wgpu = "0.20"                    # Exact version
winit = "~0.29"                  # Compatible version
anyhow = "^1.0"                  # Compatible version (default)
serde = { version = "1.0", features = ["derive"] }  # With features
tokio = { version = "1", features = ["full"] }      # With features

[dev-dependencies]
# Development-only dependencies
tokio-test = "0.4"
mockall = "0.12"
criterion = { version = "0.5", features = ["html_reports"] }

[features]
# Optional features
default = ["graphics", "audio"]
graphics = ["wgpu"]
audio = ["rodio"]

[[bench]]
name = "app_state_bench"
harness = false

[[bench]]
name = "graphics_bench"
harness = false
```

## Understanding Rust's Versioning

Rust uses semantic versioning with flexible constraints:

```toml
[dependencies]
# Exact version
exact = "1.2.3"

# Compatible version (default)
compatible = "^1.2.3"  # Same as "1.2.3"

# Tilde version
tilde = "~1.2.3"       # >= 1.2.3, < 1.3.0

# Wildcard version
wildcard = "1.2.*"     # >= 1.2.0, < 1.3.0

# Range version
range = ">= 1.2.3, < 2.0.0"

# Git dependency
git_dep = { git = "https://github.com/user/repo.git" }

# Path dependency
path_dep = { path = "../local-crate" }
```

## Understanding Rust's Feature System

Rust's feature system allows conditional compilation:

```toml
[dependencies]
wgpu = { version = "0.20", features = ["dx12", "metal"], optional = true }
rodio = { version = "0.17", optional = true }

[features]
default = ["graphics", "audio"]
graphics = ["wgpu"]
audio = ["rodio"]
```

In your code, you can use features:
```rust
#[cfg(feature = "graphics")]
mod graphics {
    use wgpu::*;
    // Graphics code here
}

#[cfg(feature = "audio")]
mod audio {
    use rodio::*;
    // Audio code here
}

#[cfg(not(feature = "graphics"))]
mod graphics {
    // Mock graphics implementation
}
```

## Understanding Rust's Workspace System

Rust supports workspaces for managing multiple related crates:

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "learn-liberty-core",
    "learn-liberty-graphics",
    "learn-liberty-audio",
    "learn-liberty-ui",
]

[workspace.dependencies]
wgpu = "0.20"
winit = "0.29"
serde = { version = "1.0", features = ["derive"] }
```

```toml
# learn-liberty-core/Cargo.toml
[package]
name = "learn-liberty-core"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { workspace = true }
```

## Understanding Rust's Build Scripts

Rust supports build scripts for custom build logic:

```rust
// build.rs
fn main() {
    // Tell Cargo to rerun this build script if the file changes
    println!("cargo:rerun-if-changed=src/");
    
    // Link against a system library
    println!("cargo:rustc-link-lib=SDL2");
    
    // Set environment variables
    println!("cargo:rustc-env=MY_VAR=value");
}
```

## Understanding Rust's Cargo Commands

Cargo provides many useful commands:

```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Run the project
cargo run

# Run tests
cargo test

# Run benchmarks
cargo bench

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Update dependencies
cargo update

# Add a new dependency
cargo add serde

# Remove a dependency
cargo remove serde

# Generate documentation
cargo doc

# Publish to crates.io
cargo publish
```

## Exercises

### Exercise 1: Add a New Dependency
Add a new dependency to `Cargo.toml` for handling configuration files. Use the `config` crate and add it with appropriate features.

**Hint**: Use `cargo add config` or manually add it to `Cargo.toml`.

### Exercise 2: Create a Feature Flag
Add a feature flag for optional audio support. Create a new dependency for audio and make it optional.

**Hint**: Use `optional = true` in the dependency and create a feature in `[features]`.

### Exercise 3: Add a Development Dependency
Add a development dependency for code coverage. Use the `tarpaulin` crate for coverage reporting.

**Hint**: Add it to `[dev-dependencies]` section.

### Exercise 4: Create a Build Script
Create a `build.rs` file that sets an environment variable with the current timestamp.

**Hint**: Use `println!("cargo:rustc-env=BUILD_TIME={}", timestamp);`.

### Exercise 5: Add a Workspace
Convert the project to a workspace with separate crates for core, graphics, and audio functionality.

**Hint**: Create separate directories and `Cargo.toml` files for each crate.

## Key Takeaways

1. **Cargo.toml**: Central configuration file for Rust projects
2. **Version Constraints**: Flexible versioning system
3. **Features**: Conditional compilation and optional dependencies
4. **Workspaces**: Managing multiple related crates
5. **Build Scripts**: Custom build logic and system integration

## Next Steps

In the next lesson, we'll explore Rust's documentation system and how it differs from Python's docstrings, while learning about the project's documentation structure.

## Resources

- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Semantic Versioning](https://semver.org/)
- [Cargo Features](https://doc.rust-lang.org/cargo/reference/features.html)
