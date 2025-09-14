# Lesson 1: Rust Basics and Project Structure

## Learning Objectives
- Understand the basic structure of a Rust project
- Learn about Rust's ownership system (coming from Python's reference system)
- Explore the Learn Liberty project organization
- Get familiar with Cargo and Rust's module system

## Rust Idiom of the Day: Ownership vs Python References

In Python, you're used to objects being passed by reference:
```python
def modify_list(my_list):
    my_list.append(42)  # Modifies the original list
    return my_list

numbers = [1, 2, 3]
result = modify_list(numbers)
print(numbers)  # [1, 2, 3, 42] - original is modified
```

In Rust, ownership is more explicit:
```rust
fn modify_vec(mut my_vec: Vec<i32>) -> Vec<i32> {
    my_vec.push(42);  // Takes ownership, can modify
    my_vec  // Returns ownership
}

let numbers = vec![1, 2, 3];
let result = modify_vec(numbers);  // numbers is moved, no longer accessible
// println!("{:?}", numbers);  // This would cause a compile error!
```

**Key Difference**: Rust's ownership system prevents data races and memory issues at compile time, while Python relies on runtime checks.

## Game Structural Concept of the Day: Application State Management

In our Learn Liberty game, the `AppState` struct manages the game's current state:

```rust
// src/state.rs
#[derive(Debug)]
pub struct AppState {
    pub frame_count: u32,
    pub time: f64,
    pub lesson_progress: f32,
    pub current_lesson_id: String,
}
```

This is similar to how you might manage game state in Python:
```python
class GameState:
    def __init__(self):
        self.frame_count = 0
        self.time = 0.0
        self.lesson_progress = 0.0
        self.current_lesson_id = ""
```

**Key Difference**: Rust's structs are more memory-efficient and provide compile-time guarantees about data layout.

## Project Structure Deep Dive

Let's explore the Learn Liberty project structure:

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
├── assets/               # Game assets (images, sounds, etc.)
├── config/               # Configuration files
├── lessons/              # Educational lessons (this folder!)
└── Cargo.toml           # Project configuration
```

## Understanding Cargo.toml

The `Cargo.toml` file is like Python's `requirements.txt` but much more powerful:

```toml
[package]
name = "learn-liberty-app"
version = "0.1.0"
edition = "2021"

[dependencies]
wgpu = { version = "0.20", features = ["dx12", "metal"] }
winit = "0.29"
anyhow = "1.0"
```

This tells Cargo:
- What your project is called
- What version it is
- What external libraries (crates) you need
- What features of those crates to enable

## Exercises

### Exercise 1: Add a New Field to AppState
Add a `player_name` field to the `AppState` struct in `src/state.rs`. Make it a `String` type and update the `Default` implementation.

**Hint**: Look at how `current_lesson_id` is implemented.

### Exercise 2: Create a Simple Function
In `src/state.rs`, add a new method to `AppState` called `get_player_info` that returns a formatted string with the player's name and current lesson.

**Hint**: Use the `format!` macro, similar to Python's f-strings.

### Exercise 3: Explore the Module System
Create a new file `src/player.rs` and add a simple `Player` struct with a `name` field. Then add `pub mod player;` to `src/lib.rs` to make it available.

**Hint**: Look at how other modules are declared in `lib.rs`.

### Exercise 4: Add a Configuration Option
In `config/settings.toml`, add a new setting for `default_player_name` and update the code to use it.

**Hint**: You'll need to add the `toml` crate to `Cargo.toml` if it's not already there.

### Exercise 5: Test Your Changes
Write a simple test in `src/state.rs` to verify your new `get_player_info` method works correctly.

**Hint**: Look at the existing tests in the file for the pattern to follow.

## Key Takeaways

1. **Ownership**: Rust's ownership system prevents many common bugs that Python developers encounter
2. **Structs**: More efficient than Python classes for simple data containers
3. **Modules**: Rust's module system is more explicit than Python's imports
4. **Cargo**: More powerful than pip for dependency management
5. **Compile-time Safety**: Many errors are caught at compile time rather than runtime

## Next Steps

In the next lesson, we'll explore Rust's type system and how it differs from Python's dynamic typing, while learning about the graphics engine architecture.

## Resources

- [Rust Book: Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust Book: Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
