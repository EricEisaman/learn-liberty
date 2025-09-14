# Lesson 2: Types and Graphics Architecture

## Learning Objectives
- Understand Rust's type system vs Python's dynamic typing
- Learn about Rust's pattern matching and enums
- Explore the graphics engine architecture
- Understand how Rust handles errors differently than Python

## Rust Idiom of the Day: Type Safety vs Python's Duck Typing

In Python, you can do this:
```python
def process_data(data):
    if isinstance(data, str):
        return data.upper()
    elif isinstance(data, int):
        return str(data * 2)
    else:
        return "unknown"

result1 = process_data("hello")  # "HELLO"
result2 = process_data(42)       # "84"
result3 = process_data([1,2,3])  # "unknown"
```

In Rust, types are checked at compile time:
```rust
enum Data {
    Text(String),
    Number(i32),
    List(Vec<i32>),
}

fn process_data(data: Data) -> String {
    match data {
        Data::Text(s) => s.to_uppercase(),
        Data::Number(n) => (n * 2).to_string(),
        Data::List(_) => "unknown".to_string(),
    }
}

let result1 = process_data(Data::Text("hello".to_string())); // "HELLO"
let result2 = process_data(Data::Number(42));                // "84"
let result3 = process_data(Data::List(vec![1,2,3]));        // "unknown"
```

**Key Difference**: Rust catches type errors at compile time, preventing runtime crashes.

## Game Structural Concept of the Day: Graphics Engine Architecture

Our graphics engine follows a layered architecture:

```rust
// src/graphics.rs
pub struct GraphicsEngine {
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
}

impl GraphicsEngine {
    pub async fn new(window: &winit::window::Window) -> anyhow::Result<Self> {
        // Initialize graphics context
    }
    
    pub fn render(&mut self) -> anyhow::Result<()> {
        // Render frame
    }
}
```

This is similar to a Python graphics framework:
```python
class GraphicsEngine:
    def __init__(self, window):
        self.width = window.width
        self.height = window.height
        self.frame_count = 0
    
    async def initialize(self):
        # Initialize graphics context
        pass
    
    def render(self):
        # Render frame
        pass
```

**Key Difference**: Rust's async/await is more explicit about ownership and error handling.

## Understanding Rust's Error Handling

Python uses exceptions:
```python
def divide(a, b):
    if b == 0:
        raise ValueError("Cannot divide by zero")
    return a / b

try:
    result = divide(10, 0)
except ValueError as e:
    print(f"Error: {e}")
```

Rust uses the `Result` type:
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

match divide(10.0, 0.0) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}
```

## Exploring the Educational Content System

Let's look at how our game handles educational content:

```rust
// src/education.rs
#[derive(Serialize, Deserialize)]
pub struct EducationalContent {
    pub id: String,
    pub title: String,
    pub description: String,
    pub media: Vec<String>,
    pub interactive_elements: Vec<InteractiveElement>,
}

#[derive(Serialize, Deserialize)]
pub struct InteractiveElement {
    pub element_type: ElementType,
    pub completion_criteria: CompletionCriteria,
}

#[derive(Serialize, Deserialize)]
pub enum ElementType {
    Quiz,
    Simulation,
    Reading,
    Video,
}
```

This is much more structured than a Python dictionary approach:
```python
educational_content = {
    "id": "lesson_1",
    "title": "Introduction",
    "description": "Learn the basics",
    "media": ["video1.mp4", "image1.png"],
    "interactive_elements": [
        {
            "type": "quiz",
            "completion_criteria": {"score": 80}
        }
    ]
}
```

## Understanding Rust's Memory Management

Rust doesn't have a garbage collector like Python. Instead, it uses ownership:

```rust
fn create_string() -> String {
    let s = String::from("Hello, World!");
    s  // Ownership is transferred to the caller
}

fn use_string(s: String) {
    println!("{}", s);
    // s is automatically dropped here
}

let my_string = create_string();
use_string(my_string);
// my_string is no longer valid here
```

In Python, this would be:
```python
def create_string():
    s = "Hello, World!"
    return s

def use_string(s):
    print(s)
    # s is still valid here

my_string = create_string()
use_string(my_string)
# my_string is still valid here
```

## Exercises

### Exercise 1: Add a New Element Type
Add a new variant to the `ElementType` enum in `src/education.rs` called `Gameplay`. Update any related code that might need to handle this new type.

**Hint**: Look at how other variants are used in the codebase.

### Exercise 2: Create a Result-Based Function
In `src/state.rs`, create a new method `set_lesson_progress` that takes a progress value and returns a `Result<(), String>`. It should return an error if the progress is not between 0.0 and 1.0.

**Hint**: Use the `if` statement for validation and `Ok(())` for success.

### Exercise 3: Implement Pattern Matching
In `src/education.rs`, create a function that takes an `ElementType` and returns a description string using pattern matching.

**Hint**: Use the `match` keyword and handle each variant.

### Exercise 4: Add Error Handling
Modify the `EducationalContent::new` function to return a `Result<Self, String>` instead of `Self`. It should return an error if the title is empty.

**Hint**: Use `if title.is_empty()` to check for empty strings.

### Exercise 5: Create a Custom Error Type
Create a new enum `EducationError` with variants for different types of errors (EmptyTitle, InvalidProgress, etc.). Update your functions to use this custom error type.

**Hint**: Look at how `anyhow::Result` is used in the codebase for inspiration.

## Key Takeaways

1. **Type Safety**: Rust catches type errors at compile time
2. **Pattern Matching**: More powerful than Python's if/elif chains
3. **Error Handling**: Explicit error handling with `Result` type
4. **Memory Management**: Ownership system prevents memory leaks
5. **Enums**: More powerful than Python's simple enums

## Next Steps

In the next lesson, we'll explore Rust's trait system and how it enables polymorphism, while learning about the window management system.

## Resources

- [Rust Book: Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust Book: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust Book: Pattern Matching](https://doc.rust-lang.org/book/ch06-02-match.html)
