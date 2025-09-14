# Learn Liberty: Rust Learning Journey

Welcome to the Learn Liberty Rust learning journey! This comprehensive course is designed for Python programmers who want to learn Rust through the lens of game development. Each lesson combines Rust language concepts with practical game development patterns.

## Course Overview

This course consists of 10 progressive lessons that will take you from Rust basics to advanced concepts, all while building understanding of the Learn Liberty educational RPG project.

## Prerequisites

- Basic Python programming experience
- Familiarity with programming concepts (variables, functions, classes)
- No prior Rust experience required

## Learning Objectives

By the end of this course, you will:

- Understand Rust's ownership system and memory safety
- Master Rust's type system and error handling
- Learn Rust's concurrency model and async programming
- Understand Rust's testing and documentation systems
- Explore Rust's performance characteristics and optimization
- Gain familiarity with Rust's ecosystem and community

## Course Structure

### [Lesson 1: Rust Basics and Project Structure](01-rust-basics-and-project-structure.md)
- **Rust Idiom**: Ownership vs Python References
- **Game Concept**: Application State Management
- **Topics**: Project structure, Cargo, modules, ownership basics
- **Exercises**: 5 hands-on exercises

### [Lesson 2: Types and Graphics Architecture](02-types-and-graphics-architecture.md)
- **Rust Idiom**: Type Safety vs Python's Duck Typing
- **Game Concept**: Graphics Engine Architecture
- **Topics**: Type system, enums, error handling, graphics pipeline
- **Exercises**: 5 hands-on exercises

### [Lesson 3: Traits and Window Management](03-traits-and-window-management.md)
- **Rust Idiom**: Traits vs Python's Duck Typing
- **Game Concept**: Window Management System
- **Topics**: Traits, borrowing, lifetimes, window events
- **Exercises**: 5 hands-on exercises

### [Lesson 4: Collections and Graphics Pipeline](04-collections-and-graphics-pipeline.md)
- **Rust Idiom**: Collections vs Python Lists/Dicts
- **Game Concept**: Graphics Rendering Pipeline
- **Topics**: Collections, iterators, strings, Option type
- **Exercises**: 5 hands-on exercises

### [Lesson 5: Concurrency and Main Loop](05-concurrency-and-main-loop.md)
- **Rust Idiom**: Concurrency vs Python Threading
- **Game Concept**: Main Loop Architecture
- **Topics**: Async/await, Send/Sync, channels, main loop
- **Exercises**: 5 hands-on exercises

### [Lesson 6: Testing and Quality Assurance](06-testing-and-quality-assurance.md)
- **Rust Idiom**: Testing vs Python's unittest
- **Game Concept**: Testing Architecture
- **Topics**: Unit tests, integration tests, mocking, benchmarking
- **Exercises**: 5 hands-on exercises

### [Lesson 7: Package Management and Dependencies](07-package-management-and-dependencies.md)
- **Rust Idiom**: Cargo vs Python Package Management
- **Game Concept**: Dependency Architecture
- **Topics**: Cargo.toml, versioning, features, workspaces
- **Exercises**: 5 hands-on exercises

### [Lesson 8: Documentation and Code Organization](08-documentation-and-code-organization.md)
- **Rust Idiom**: Documentation vs Python Docstrings
- **Game Concept**: Module Organization
- **Topics**: Documentation comments, modules, visibility, API design
- **Exercises**: 5 hands-on exercises

### [Lesson 9: Performance and Optimization](09-performance-and-optimization.md)
- **Rust Idiom**: Zero-Cost Abstractions vs Python's Overhead
- **Game Concept**: Performance Optimization
- **Topics**: Memory management, benchmarking, profiling, optimization
- **Exercises**: 5 hands-on exercises

### [Lesson 10: Ecosystem and Future Development](10-ecosystem-and-future-development.md)
- **Rust Idiom**: Ecosystem vs Python's Package Index
- **Game Concept**: Future Development
- **Topics**: Rust ecosystem, community, contributing, learning path
- **Exercises**: 5 hands-on exercises

## How to Use This Course

1. **Read Each Lesson**: Start with Lesson 1 and work through them sequentially
2. **Complete Exercises**: Each lesson includes 5 practical exercises
3. **Experiment**: Try modifying the code examples
4. **Ask Questions**: Use the Rust community resources for help
5. **Practice**: Build your own projects using the concepts learned

## Getting Started

1. **Install Rust**: Follow the instructions at [rustup.rs](https://rustup.rs/)
2. **Clone the Project**: `git clone <repository-url>`
3. **Build the Project**: `cargo build`
4. **Run Tests**: `cargo test`
5. **Start Learning**: Begin with [Lesson 1](01-rust-basics-and-project-structure.md)

## Project Structure

```
learn-liberty/
├── src/                    # Source code
│   ├── main.rs            # Application entry point
│   ├── lib.rs             # Library interface
│   ├── window.rs          # Window management
│   ├── graphics.rs        # Graphics engine
│   ├── state.rs           # Game state management
│   └── education.rs       # Educational content system
├── tests/                 # Integration tests
├── benches/              # Performance benchmarks
├── assets/               # Game assets
├── config/               # Configuration files
├── lessons/              # This course!
└── Cargo.toml           # Project configuration
```

## Key Differences from Python

| Concept | Python | Rust |
|---------|--------|------|
| Memory Management | Garbage Collection | Ownership System |
| Type System | Dynamic | Static |
| Error Handling | Exceptions | Result/Option |
| Concurrency | Threading | Async/Await |
| Package Management | pip | Cargo |
| Performance | Interpreted | Compiled |

## Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Official Rust documentation
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive Rust exercises
- [Exercism Rust Track](https://exercism.org/tracks/rust) - Practice problems
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by example
- [Rust Community](https://www.rust-lang.org/community) - Get help and connect

## Contributing

This course is part of the Learn Liberty project. Contributions are welcome! Please see the main project README for contribution guidelines.

## License

This course is licensed under the same license as the Learn Liberty project. See the main project LICENSE file for details.

## Acknowledgments

- The Rust community for creating an amazing language
- The Learn Liberty project contributors
- All the educators who make learning accessible

---

**Happy Learning!** 🦀

Start your Rust journey with [Lesson 1: Rust Basics and Project Structure](01-rust-basics-and-project-structure.md)
