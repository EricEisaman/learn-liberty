//! Learn Liberty - Educational RPG
//!
//! A Rust-based educational graphics application inspired by the Handmaid's tale
//! by Casey Moriarty. This is a simple 2D four-way scroller RPG designed for
//! educational content delivery.

pub mod education;
pub mod graphics;
pub mod state;
pub mod window;

// Re-export main types for easier access
pub use education::{CompletionCriteria, EducationalContent, ElementType, InteractiveElement};
pub use graphics::GraphicsEngine;
pub use state::AppState;
pub use window::WindowManager;

pub mod tests {
    //! Test utilities and mock objects
    //!
    //! This module provides mock implementations and test utilities
    //! for testing the Learn Liberty educational graphics application.

    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    /// Mock window for testing window-related functionality
    pub struct MockWindow {
        pub title: String,
        pub width: u32,
        pub height: u32,
        pub events: mpsc::Receiver<MockEvent>,
    }

    /// Mock events for testing
    #[derive(Debug, Clone)]
    pub enum MockEvent {
        Close,
        Resize { width: u32, height: u32 },
        KeyPress { key: String },
        MouseClick { x: f32, y: f32 },
    }

    impl MockWindow {
        pub fn new(title: &str, width: u32, height: u32) -> (Self, mpsc::Sender<MockEvent>) {
            let (tx, rx) = mpsc::channel();

            let window = Self {
                title: title.to_string(),
                width,
                height,
                events: rx,
            };

            (window, tx)
        }

        pub fn get_title(&self) -> &str {
            &self.title
        }

        pub fn get_size(&self) -> (u32, u32) {
            (self.width, self.height)
        }

        pub fn try_receive_event(&self) -> Option<MockEvent> {
            self.events.try_recv().ok()
        }
    }

    /// Mock graphics engine for testing graphics functionality
    pub struct MockGraphicsEngine {
        pub width: u32,
        pub height: u32,
        pub frame_count: u32,
        pub render_calls: u32,
    }

    impl MockGraphicsEngine {
        pub fn new(width: u32, height: u32) -> Self {
            Self {
                width,
                height,
                frame_count: 0,
                render_calls: 0,
            }
        }

        pub fn render(&mut self) -> Result<(), String> {
            self.render_calls += 1;
            self.frame_count += 1;

            // Simulate some rendering work
            std::thread::sleep(Duration::from_millis(1));

            Ok(())
        }

        pub fn resize(&mut self, width: u32, height: u32) {
            self.width = width;
            self.height = height;
        }

        pub fn get_frame_count(&self) -> u32 {
            self.frame_count
        }

        pub fn get_render_calls(&self) -> u32 {
            self.render_calls
        }
    }

    /// Test utilities for creating test data
    pub struct TestUtils;

    impl TestUtils {
        /// Create a test app state with default values
        pub fn create_test_app_state() -> AppState {
            AppState::default()
        }

        /// Create a test educational content
        pub fn create_test_lesson() -> EducationalContent {
            EducationalContent::new(
                "test_lesson_1".to_string(),
                "Test Lesson 1".to_string(),
                "This is a test lesson for educational content".to_string(),
            )
        }

        /// Create multiple test lessons
        pub fn create_test_lessons(count: usize) -> Vec<EducationalContent> {
            (0..count)
                .map(|i| {
                    EducationalContent::new(
                        format!("test_lesson_{}", i + 1),
                        format!("Test Lesson {}", i + 1),
                        format!("This is test lesson number {}", i + 1),
                    )
                })
                .collect()
        }

        /// Simulate frame updates for testing
        pub fn simulate_frame_updates(state: &mut AppState, frames: u32, delta_time: f64) {
            for _ in 0..frames {
                state.update(delta_time);
            }
        }
    }

    /// Performance testing utilities
    pub struct PerformanceTest {
        pub start_time: std::time::Instant,
        pub measurements: Vec<Duration>,
    }

    impl Default for PerformanceTest {
        fn default() -> Self {
            Self::new()
        }
    }

    impl PerformanceTest {
        pub fn new() -> Self {
            Self {
                start_time: std::time::Instant::now(),
                measurements: Vec::new(),
            }
        }

        pub fn measure<F>(&mut self, f: F) -> Duration
        where
            F: FnOnce(),
        {
            let start = std::time::Instant::now();
            f();
            let duration = start.elapsed();
            self.measurements.push(duration);
            duration
        }

        pub fn get_average_time(&self) -> Duration {
            if self.measurements.is_empty() {
                Duration::from_secs(0)
            } else {
                let total: Duration = self.measurements.iter().sum();
                total / self.measurements.len() as u32
            }
        }

        pub fn get_total_time(&self) -> Duration {
            self.start_time.elapsed()
        }
    }
}
