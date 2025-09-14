# Lesson 5: Concurrency and Main Loop

## Learning Objectives
- Understand Rust's concurrency model vs Python's threading
- Learn about async/await in Rust
- Explore the application's main loop architecture
- Understand how Rust handles shared state safely

## Rust Idiom of the Day: Concurrency vs Python Threading

In Python, you use threading for concurrency:
```python
import threading
import time

def worker(name, delay):
    for i in range(5):
        print(f"Worker {name}: {i}")
        time.sleep(delay)

# Create threads
thread1 = threading.Thread(target=worker, args=("A", 1))
thread2 = threading.Thread(target=worker, args=("B", 0.5))

# Start threads
thread1.start()
thread2.start()

# Wait for completion
thread1.join()
thread2.join()
```

In Rust, you use async/await for concurrency:
```rust
use tokio::time::{sleep, Duration};

async fn worker(name: &str, delay: u64) {
    for i in 0..5 {
        println!("Worker {}: {}", name, i);
        sleep(Duration::from_millis(delay)).await;
    }
}

#[tokio::main]
async fn main() {
    // Run tasks concurrently
    tokio::join!(
        worker("A", 1000),
        worker("B", 500)
    );
}
```

**Key Difference**: Rust's async/await is more efficient and safer than Python's threading.

## Game Structural Concept of the Day: Main Loop Architecture

Our application's main loop follows the game loop pattern:

```rust
// src/main.rs
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    let window_manager = WindowManager::new("Learn Liberty - Educational RPG");
    let window = window_manager.window();
    let mut graphics_engine = GraphicsEngine::new(&window).await?;
    let mut app_state = AppState::default();
    
    let mut last_time = Instant::now();
    
    window_manager.run(move |_window| {
        let current_time = Instant::now();
        let delta_time = last_time.elapsed().as_secs_f64();
        last_time = current_time;
        
        // Update game state
        app_state.update(delta_time);
        
        // Render frame
        if let Err(e) = graphics_engine.render() {
            eprintln!("Render error: {}", e);
        }
    });
    
    Ok(())
}
```

This is similar to a Python game loop:
```python
import time

def main():
    window_manager = WindowManager("Learn Liberty - Educational RPG")
    graphics_engine = GraphicsEngine(window_manager.window)
    app_state = AppState()
    
    last_time = time.time()
    
    while window_manager.running:
        current_time = time.time()
        delta_time = current_time - last_time
        last_time = current_time
        
        # Update game state
        app_state.update(delta_time)
        
        # Render frame
        try:
            graphics_engine.render()
        except Exception as e:
            print(f"Render error: {e}")
        
        window_manager.process_events()
```

**Key Difference**: Rust's ownership system prevents data races in the main loop.

## Understanding Rust's Async/Await

Rust's async/await is built on top of futures:

```rust
use std::future::Future;

async fn fetch_data() -> String {
    // Simulate async work
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    "Data fetched".to_string()
}

async fn process_data() -> String {
    let data = fetch_data().await;
    format!("Processed: {}", data)
}

#[tokio::main]
async fn main() {
    let result = process_data().await;
    println!("{}", result);
}
```

In Python, this would be:
```python
import asyncio

async def fetch_data():
    # Simulate async work
    await asyncio.sleep(1)
    return "Data fetched"

async def process_data():
    data = await fetch_data()
    return f"Processed: {data}"

async def main():
    result = await process_data()
    print(result)

asyncio.run(main())
```

## Understanding Rust's Send and Sync Traits

Rust uses `Send` and `Sync` traits to ensure thread safety:

- **`Send`**: Can be transferred between threads
- **`Sync`**: Can be shared between threads

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Arc<T> provides shared ownership
// Mutex<T> provides mutual exclusion
let data = Arc::new(Mutex::new(0));

let handles: Vec<_> = (0..10).map(|i| {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        let mut num = data.lock().unwrap();
        *num += i;
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *data.lock().unwrap());
```

In Python, you'd use similar concepts:
```python
import threading

data = 0
lock = threading.Lock()

def worker(i):
    global data
    with lock:
        data += i

threads = []
for i in range(10):
    t = threading.Thread(target=worker, args=(i,))
    threads.append(t)
    t.start()

for t in threads:
    t.join()

print(f"Result: {data}")
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
                WindowEvent::Resized(size) => {
                    // Handle window resize
                }
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

The `'static` lifetime bound ensures the closure can live for the entire program duration.

## Understanding Rust's Channel System

Rust provides channels for communication between threads:

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

// Spawn a thread to send data
thread::spawn(move || {
    let values = vec![1, 2, 3, 4, 5];
    for val in values {
        tx.send(val).unwrap();
    }
});

// Receive data in the main thread
for received in rx {
    println!("Got: {}", received);
}
```

In Python, you'd use `queue.Queue`:
```python
import queue
import threading

q = queue.Queue()

def worker():
    values = [1, 2, 3, 4, 5]
    for val in values:
        q.put(val)

t = threading.Thread(target=worker)
t.start()

while True:
    try:
        received = q.get(timeout=1)
        print(f"Got: {received}")
    except queue.Empty:
        break
```

## Exercises

### Exercise 1: Create an Async Function
In `src/state.rs`, create an async function `load_lesson_data` that simulates loading lesson data from a file. Use `tokio::time::sleep` to simulate the delay.

**Hint**: Use `async fn` and `await` keywords.

### Exercise 2: Implement Thread-Safe State Updates
Add a method to `AppState` that can be safely called from multiple threads. Use `Arc<Mutex<AppState>>` for thread safety.

**Hint**: Look at how `Arc` and `Mutex` are used in the examples.

### Exercise 3: Create a Channel for Events
Create a channel system in `src/window.rs` to send window events to the main thread. Use `mpsc::channel` for communication.

**Hint**: Use `std::sync::mpsc` for the channel implementation.

### Exercise 4: Implement Async Asset Loading
Create an async function in `src/education.rs` that loads educational content from multiple files concurrently using `tokio::join!`.

**Hint**: Use `tokio::fs::read_to_string` for file operations.

### Exercise 5: Add Error Handling to Async Functions
Modify your async functions to return `Result<T, Box<dyn std::error::Error + Send + Sync>>` and handle errors properly.

**Hint**: Use `Box<dyn std::error::Error + Send + Sync>` for error types that can be sent between threads.

## Key Takeaways

1. **Async/Await**: More efficient than threading for I/O-bound tasks
2. **Send/Sync**: Compile-time guarantees for thread safety
3. **Channels**: Safe communication between threads
4. **Ownership**: Prevents data races at compile time
5. **Error Handling**: Explicit error handling in async code

## Next Steps

In the next lesson, we'll explore Rust's testing framework and how it differs from Python's unittest, while learning about the project's testing architecture.

## Resources

- [Rust Book: Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Book: Async Programming](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
