# Lesson 4: Collections and Graphics Pipeline

## Learning Objectives
- Understand Rust's collections vs Python's built-in types
- Learn about Rust's iterator system
- Explore the graphics rendering pipeline
- Understand how Rust handles memory-efficient data structures

## Rust Idiom of the Day: Collections vs Python Lists/Dicts

In Python, you're used to dynamic lists and dictionaries:
```python
# Python lists are dynamic and can hold any type
numbers = [1, 2, 3, 4, 5]
numbers.append(6)
numbers.extend([7, 8, 9])

# Python dictionaries are flexible
player_data = {
    "name": "Alice",
    "score": 100,
    "items": ["sword", "potion"]
}
player_data["level"] = 5
```

In Rust, collections are more specific and memory-efficient:
```rust
// Vec<T> is like Python's list but type-specific
let mut numbers = vec![1, 2, 3, 4, 5];
numbers.push(6);
numbers.extend([7, 8, 9]);

// HashMap<K, V> is like Python's dict but type-specific
use std::collections::HashMap;
let mut player_data = HashMap::new();
player_data.insert("name".to_string(), "Alice".to_string());
player_data.insert("score".to_string(), 100);
player_data.insert("level".to_string(), 5);
```

**Key Difference**: Rust collections are type-safe and more memory-efficient.

## Game Structural Concept of the Day: Graphics Rendering Pipeline

Our graphics engine follows a typical rendering pipeline:

```rust
// src/graphics.rs
pub struct GraphicsEngine {
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
}

impl GraphicsEngine {
    pub fn render(&mut self) -> anyhow::Result<()> {
        // 1. Clear the screen
        self.clear_screen()?;
        
        // 2. Update frame count
        self.frame_count += 1;
        
        // 3. Render game objects
        self.render_objects()?;
        
        // 4. Present the frame
        self.present_frame()?;
        
        Ok(())
    }
    
    fn clear_screen(&self) -> anyhow::Result<()> {
        // Clear to black
        Ok(())
    }
    
    fn render_objects(&self) -> anyhow::Result<()> {
        // Render all game objects
        Ok(())
    }
    
    fn present_frame(&self) -> anyhow::Result<()> {
        // Present the rendered frame
        Ok(())
    }
}
```

This is similar to a Python graphics pipeline:
```python
class GraphicsEngine:
    def __init__(self):
        self.width = 800
        self.height = 600
        self.frame_count = 0
    
    def render(self):
        # 1. Clear the screen
        self.clear_screen()
        
        # 2. Update frame count
        self.frame_count += 1
        
        # 3. Render game objects
        self.render_objects()
        
        # 4. Present the frame
        self.present_frame()
    
    def clear_screen(self):
        # Clear to black
        pass
    
    def render_objects(self):
        # Render all game objects
        pass
    
    def present_frame(self):
        # Present the rendered frame
        pass
```

**Key Difference**: Rust's error handling with `Result` makes the pipeline more robust.

## Understanding Rust's Iterator System

Rust's iterators are lazy and memory-efficient:

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Chain operations without creating intermediate collections
let result: Vec<i32> = numbers
    .iter()                    // Create iterator
    .filter(|&x| x % 2 == 0)   // Keep only even numbers
    .map(|x| x * 2)            // Double each number
    .collect();                // Collect into Vec

println!("{:?}", result); // [4, 8]
```

In Python, this would be:
```python
numbers = [1, 2, 3, 4, 5]

# List comprehensions are more Pythonic
result = [x * 2 for x in numbers if x % 2 == 0]

print(result)  # [4, 8]
```

**Key Difference**: Rust iterators are lazy and don't create intermediate collections.

## Exploring the Educational Content System

Our educational content system uses collections effectively:

```rust
// src/education.rs
#[derive(Serialize, Deserialize)]
pub struct EducationalContent {
    pub id: String,
    pub title: String,
    pub description: String,
    pub media: Vec<String>,                    // Collection of media files
    pub interactive_elements: Vec<InteractiveElement>, // Collection of elements
}

impl EducationalContent {
    pub fn add_media(&mut self, media_path: String) {
        self.media.push(media_path);
    }
    
    pub fn add_interactive_element(&mut self, element: InteractiveElement) {
        self.interactive_elements.push(element);
    }
}
```

This is more structured than Python's approach:
```python
class EducationalContent:
    def __init__(self):
        self.media = []
        self.interactive_elements = []
    
    def add_media(self, media_path):
        self.media.append(media_path)
    
    def add_interactive_element(self, element):
        self.interactive_elements.append(element)
```

## Understanding Rust's String Types

Rust has two main string types:

1. **`String`**: Owned, growable string
2. **`&str`**: String slice, borrowed reference

```rust
// String - owned
let owned_string = String::from("Hello, World!");
let owned_string2 = "Hello, World!".to_string();

// &str - borrowed
let string_slice = "Hello, World!";
let string_slice2 = &owned_string;

// Conversion between types
let from_slice = string_slice.to_string();
let to_slice = &owned_string;
```

In Python, strings are immutable and there's only one type:
```python
# Python strings are immutable
text = "Hello, World!"
text2 = str("Hello, World!")

# No conversion needed
same_text = text
```

## Understanding Rust's Option Type

Rust uses `Option<T>` instead of `None`:

```rust
fn find_element(elements: &[String], target: &str) -> Option<usize> {
    for (index, element) in elements.iter().enumerate() {
        if element == target {
            return Some(index);
        }
    }
    None
}

let elements = vec!["apple".to_string(), "banana".to_string(), "cherry".to_string()];
match find_element(&elements, "banana") {
    Some(index) => println!("Found at index: {}", index),
    None => println!("Not found"),
}
```

In Python, you'd use `None`:
```python
def find_element(elements, target):
    for index, element in enumerate(elements):
        if element == target:
            return index
    return None

elements = ["apple", "banana", "cherry"]
result = find_element(elements, "banana")
if result is not None:
    print(f"Found at index: {result}")
else:
    print("Not found")
```

## Exercises

### Exercise 1: Create a Collection of Game Objects
In `src/education.rs`, create a new struct `GameObject` with a `name` field and implement a collection of game objects in `EducationalContent`.

**Hint**: Use `Vec<GameObject>` for the collection.

### Exercise 2: Implement Iterator for Educational Content
Create an iterator that yields all the media files in an `EducationalContent` struct. Use the `Iterator` trait.

**Hint**: Look at how iterators are implemented in the Rust standard library.

### Exercise 3: Add String Processing
Create a function that takes a `Vec<String>` and returns a new `Vec<String>` with all strings converted to uppercase, using iterators.

**Hint**: Use `map` and `collect` methods.

### Exercise 4: Implement Option-Based Search
Create a function that searches for an interactive element by type in `EducationalContent` and returns an `Option<&InteractiveElement>`.

**Hint**: Use `iter()` and `find()` methods.

### Exercise 5: Create a HashMap for Game State
In `src/state.rs`, add a `HashMap<String, String>` field to store player preferences and implement methods to get and set preferences.

**Hint**: Use `std::collections::HashMap` and handle the `Option` return type.

## Key Takeaways

1. **Collections**: Type-safe and memory-efficient
2. **Iterators**: Lazy and composable
3. **Strings**: Two types with different ownership semantics
4. **Option**: Explicit handling of missing values
5. **Memory Efficiency**: No garbage collection overhead

## Next Steps

In the next lesson, we'll explore Rust's concurrency model and how it differs from Python's threading, while learning about the application's main loop.

## Resources

- [Rust Book: Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Rust Book: Iterators](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- [Rust Book: Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
