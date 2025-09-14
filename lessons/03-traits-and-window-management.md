# Lesson 3: Traits and Window Management

## Learning Objectives
- Understand Rust's trait system vs Python's duck typing
- Learn about trait bounds and generic programming
- Explore the window management system
- Understand how Rust handles polymorphism

## Rust Idiom of the Day: Traits vs Python's Duck Typing

In Python, you can use duck typing:
```python
class Duck:
    def quack(self):
        return "Quack!"

class Person:
    def quack(self):
        return "I'm not a duck, but I can quack!"

def make_sound(animal):
    return animal.quack()  # Works with any object that has quack()

duck = Duck()
person = Person()
print(make_sound(duck))   # "Quack!"
print(make_sound(person)) # "I'm not a duck, but I can quack!"
```

In Rust, you use traits for similar functionality:
```rust
trait Quackable {
    fn quack(&self) -> String;
}

struct Duck;
impl Quackable for Duck {
    fn quack(&self) -> String {
        "Quack!".to_string()
    }
}

struct Person;
impl Quackable for Person {
    fn quack(&self) -> String {
        "I'm not a duck, but I can quack!".to_string()
    }
}

fn make_sound<T: Quackable>(animal: T) -> String {
    animal.quack()
}

let duck = Duck;
let person = Person;
println!("{}", make_sound(duck));   // "Quack!"
println!("{}", make_sound(person)); // "I'm not a duck, but I can quack!"
```

**Key Difference**: Rust's traits provide compile-time guarantees about what methods are available.

## Game Structural Concept of the Day: Window Management System

Our window management system uses Rust's ownership and borrowing:

```rust
// src/window.rs
pub struct WindowManager {
    window: winit::window::Window,
    event_loop: EventLoop<()>,
}

impl WindowManager {
    pub fn new(title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap();

        Self { window, event_loop }
    }
    
    pub fn run<F>(self, mut update: F)
    where
        F: FnMut(&winit::window::Window) + 'static,
    {
        // Event loop implementation
    }
}
```

This is similar to a Python window manager:
```python
class WindowManager:
    def __init__(self, title):
        self.window = create_window(title)
        self.event_loop = create_event_loop()
    
    def run(self, update_callback):
        while self.running:
            update_callback(self.window)
            self.event_loop.process_events()
```

**Key Difference**: Rust's closure system is more explicit about lifetime and ownership.

## Understanding Rust's Borrowing System

Rust has three types of references:
1. **Immutable reference** (`&T`): Read-only access
2. **Mutable reference** (`&mut T`): Read-write access
3. **Owned value** (`T`): Full ownership

```rust
fn process_data(data: &String) -> String {
    data.to_uppercase()  // Can read but not modify
}

fn modify_data(data: &mut String) {
    data.push_str(" modified");  // Can read and modify
}

fn take_ownership(data: String) -> String {
    data  // Takes ownership, can do anything
}

let mut text = String::from("Hello");
let result1 = process_data(&text);        // Borrow immutably
modify_data(&mut text);                   // Borrow mutably
let result2 = take_ownership(text);       // Take ownership
// text is no longer valid here
```

In Python, this distinction doesn't exist:
```python
def process_data(data):
    return data.upper()  # Can read

def modify_data(data):
    data.append(" modified")  # Can modify

def take_ownership(data):
    return data  # Still valid after function

text = "Hello"
result1 = process_data(text)  # text still valid
modify_data(text)             # text still valid
result2 = take_ownership(text)  # text still valid
```

## Exploring the Event System

Our window manager uses Rust's closure system for event handling:

```rust
pub fn run<F>(self, mut update: F)
where
    F: FnMut(&winit::window::Window) + 'static,
{
    let _ = self.event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => elwt.exit(),
                _ => (),
            },
            Event::AboutToWait => {
                update(&self.window);
                self.window.request_redraw();
            }
            _ => (),
        }
    });
}
```

The `where` clause is a trait bound that says "F must be a function that can be called multiple times with a window reference and has a static lifetime."

## Understanding Lifetime Parameters

Rust's lifetime system ensures references are valid:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

let string1 = String::from("long string is long");
let result;
{
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
}  // string2 goes out of scope here
// result is still valid because it points to string1
```

## Exercises

### Exercise 1: Create a Trait for Game Objects
Create a new trait `GameObject` in `src/education.rs` with methods `update(&mut self, delta_time: f64)` and `render(&self)`. Then implement this trait for `EducationalContent`.

**Hint**: Look at how other traits are defined in the codebase.

### Exercise 2: Add a Generic Window Event Handler
Create a generic function in `src/window.rs` that can handle different types of window events. Use trait bounds to ensure the handler can process events.

**Hint**: Use `FnMut` trait bound for the handler function.

### Exercise 3: Implement a Borrowing Pattern
In `src/state.rs`, create a method that borrows the `AppState` immutably to read data and another that borrows it mutably to update data.

**Hint**: Use `&self` for immutable borrowing and `&mut self` for mutable borrowing.

### Exercise 4: Create a Lifetime-Aware Function
Create a function that takes two string references with the same lifetime and returns the longer one. Handle the case where they're equal length.

**Hint**: Use lifetime parameters like `<'a>` and return `&'a str`.

### Exercise 5: Implement a Trait Object
Create a trait `Renderable` and implement it for different types of game objects. Then create a vector of trait objects that can be rendered.

**Hint**: Use `Box<dyn Renderable>` for trait objects.

## Key Takeaways

1. **Traits**: Provide compile-time polymorphism
2. **Borrowing**: Three types of references with different capabilities
3. **Lifetimes**: Ensure references are valid
4. **Closures**: More explicit than Python's lambda functions
5. **Trait Bounds**: Constrain generic types

## Next Steps

In the next lesson, we'll explore Rust's collections and iterators, while learning about the graphics rendering pipeline.

## Resources

- [Rust Book: Traits](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [Rust Book: Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust Book: Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
