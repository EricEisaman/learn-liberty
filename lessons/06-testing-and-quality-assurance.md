# Lesson 6: Testing and Quality Assurance

## Learning Objectives
- Understand Rust's testing framework vs Python's unittest
- Learn about different types of tests in Rust
- Explore the project's testing architecture
- Understand how Rust ensures code quality

## Rust Idiom of the Day: Testing vs Python's unittest

In Python, you use unittest or pytest:
```python
import unittest

class TestAppState(unittest.TestCase):
    def setUp(self):
        self.app_state = AppState()
    
    def test_default_values(self):
        self.assertEqual(self.app_state.frame_count, 0)
        self.assertEqual(self.app_state.time, 0.0)
    
    def test_update(self):
        self.app_state.update(0.016)
        self.assertEqual(self.app_state.frame_count, 1)
        self.assertAlmostEqual(self.app_state.time, 0.016, places=5)

if __name__ == '__main__':
    unittest.main()
```

In Rust, you use the built-in testing framework:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        assert_eq!(state.frame_count, 0);
        assert_eq!(state.time, 0.0);
    }

    #[test]
    fn test_app_state_update() {
        let mut state = AppState::default();
        state.update(0.016);
        assert_eq!(state.frame_count, 1);
        assert!((state.time - 0.016).abs() < 1e-10);
    }
}
```

**Key Difference**: Rust's testing is built into the language and runs with `cargo test`.

## Game Structural Concept of the Day: Testing Architecture

Our project has a comprehensive testing architecture:

```
tests/
├── lib.rs              # Test utilities and mock objects
├── integration.rs      # Integration tests
└── graphics_tests.rs   # Graphics-specific tests

src/
├── state.rs            # Unit tests for AppState
├── education.rs        # Unit tests for EducationalContent
└── lib.rs              # Test utilities
```

This is similar to Python's testing structure:
```
tests/
├── test_state.py       # Unit tests for AppState
├── test_education.py   # Unit tests for EducationalContent
└── test_integration.py # Integration tests
```

**Key Difference**: Rust's testing is more integrated with the build system.

## Understanding Rust's Testing Framework

Rust has three types of tests:

1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test the public API of your crate
3. **Documentation Tests**: Test code examples in documentation

```rust
// Unit test (in src/state.rs)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut state = AppState::default();
        state.update(0.016);
        assert_eq!(state.frame_count, 1);
    }
}

// Integration test (in tests/integration.rs)
use learn_liberty_app::{AppState, EducationalContent};

#[test]
fn test_app_state_with_educational_content() {
    let mut state = AppState::default();
    let content = EducationalContent::new(
        "lesson_1".to_string(),
        "Introduction".to_string(),
        "Learn the basics".to_string(),
    );
    
    state.advance_lesson("lesson_1".to_string(), 0.5);
    assert_eq!(state.current_lesson_id, "lesson_1");
}

// Documentation test (in src/state.rs)
/// # Examples
/// ```
/// use learn_liberty_app::AppState;
/// let mut state = AppState::default();
/// state.update(0.016);
/// assert_eq!(state.frame_count, 1);
/// ```
impl AppState {
    pub fn update(&mut self, delta_time: f64) {
        self.frame_count += 1;
        self.time += delta_time;
    }
}
```

## Understanding Mock Objects in Rust

Our project uses mock objects for testing:

```rust
// tests/lib.rs
use mockall::*;

#[automock]
pub trait Window {
    fn get_size(&self) -> (u32, u32);
    fn set_title(&mut self, title: &str);
}

#[automock]
pub trait GraphicsEngine {
    fn render(&mut self) -> Result<(), String>;
    fn resize(&mut self, width: u32, height: u32);
}

#[test]
fn test_mock_window() {
    let mut mock_window = MockWindow::new();
    mock_window.expect_get_size()
        .return_const((800, 600));
    
    assert_eq!(mock_window.get_size(), (800, 600));
}
```

In Python, you'd use unittest.mock:
```python
from unittest.mock import Mock, MagicMock

def test_mock_window():
    mock_window = Mock()
    mock_window.get_size.return_value = (800, 600)
    
    assert mock_window.get_size() == (800, 600)
```

## Understanding Rust's Error Handling in Tests

Rust's `Result` type makes error handling explicit in tests:

```rust
#[test]
fn test_result_handling() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = AppState::default();
    state.update(0.016)?;  // This would fail if update returned an error
    
    assert_eq!(state.frame_count, 1);
    Ok(())
}

#[test]
fn test_error_handling() {
    let result = divide(10.0, 0.0);
    assert!(result.is_err());
    
    if let Err(e) = result {
        assert_eq!(e, "Cannot divide by zero");
    }
}
```

In Python, you'd use try/except:
```python
def test_result_handling():
    state = AppState()
    state.update(0.016)  # This would raise an exception if it failed
    assert state.frame_count == 1

def test_error_handling():
    with pytest.raises(ValueError, match="Cannot divide by zero"):
        divide(10.0, 0.0)
```

## Understanding Rust's Benchmarking

Rust has built-in benchmarking support:

```rust
// benches/app_state_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use learn_liberty_app::AppState;

fn benchmark_app_state_update(c: &mut Criterion) {
    c.bench_function("app_state_update", |b| {
        let mut state = AppState::default();
        b.iter(|| {
            state.update(black_box(0.016));
        });
    });
}

criterion_group!(benches, benchmark_app_state_update);
criterion_main!(benches);
```

In Python, you'd use timeit or pytest-benchmark:
```python
import timeit

def benchmark_app_state_update():
    state = AppState()
    timeit.timeit(lambda: state.update(0.016), number=1000)
```

## Understanding Rust's Property-Based Testing

Rust supports property-based testing with the `proptest` crate:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_app_state_update_properties(delta_time in 0.0f64..1.0f64) {
        let mut state = AppState::default();
        let initial_frame_count = state.frame_count;
        let initial_time = state.time;
        
        state.update(delta_time);
        
        assert_eq!(state.frame_count, initial_frame_count + 1);
        assert!((state.time - (initial_time + delta_time)).abs() < 1e-10);
    }
}
```

## Exercises

### Exercise 1: Write Unit Tests for EducationalContent
In `src/education.rs`, add unit tests for the `EducationalContent` struct. Test the `new`, `add_media`, and `add_interactive_element` methods.

**Hint**: Use `#[cfg(test)]` and `#[test]` attributes.

### Exercise 2: Create Integration Tests
In `tests/integration.rs`, create integration tests that test the interaction between `AppState` and `EducationalContent`.

**Hint**: Use `use learn_liberty_app::*;` to import the public API.

### Exercise 3: Implement Mock Objects
Create mock objects for the `GraphicsEngine` and `WindowManager` in `tests/lib.rs` using the `mockall` crate.

**Hint**: Use `#[automock]` attribute on traits.

### Exercise 4: Add Error Handling Tests
Write tests that verify error handling in your functions. Test both success and failure cases.

**Hint**: Use `assert!(result.is_ok())` and `assert!(result.is_err())`.

### Exercise 5: Create Performance Tests
Add benchmark tests for critical functions in your code. Use the `criterion` crate for benchmarking.

**Hint**: Use `c.bench_function` and `black_box` for benchmarking.

## Key Takeaways

1. **Built-in Testing**: Rust's testing framework is integrated with the language
2. **Three Types of Tests**: Unit, integration, and documentation tests
3. **Mock Objects**: Use `mockall` crate for creating mock objects
4. **Error Handling**: Explicit error handling in tests with `Result` type
5. **Benchmarking**: Built-in support for performance testing

## Next Steps

In the next lesson, we'll explore Rust's package management and how it differs from Python's pip, while learning about the project's dependency management.

## Resources

- [Rust Book: Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Criterion Documentation](https://docs.rs/criterion/)
- [Mockall Documentation](https://docs.rs/mockall/)
