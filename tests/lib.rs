//! Test utilities and mock objects for Learn Liberty
//!
//! This module provides mock implementations and test utilities
//! for testing the Learn Liberty educational graphics application.

use learn_liberty_app::{education::EducationalContent, state::AppState};
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

// PerformanceTest is now defined in src/lib.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_window_creation() {
        let (window, _sender) = MockWindow::new("Test Window", 800, 600);

        assert_eq!(window.get_title(), "Test Window");
        assert_eq!(window.get_size(), (800, 600));
    }

    #[test]
    fn test_mock_graphics_engine() {
        let mut engine = MockGraphicsEngine::new(1024, 768);

        assert_eq!(engine.width, 1024);
        assert_eq!(engine.height, 768);
        assert_eq!(engine.get_frame_count(), 0);
        assert_eq!(engine.get_render_calls(), 0);

        let result = engine.render();
        assert!(result.is_ok());
        assert_eq!(engine.get_frame_count(), 1);
        assert_eq!(engine.get_render_calls(), 1);
    }

    #[test]
    fn test_graphics_engine_resize() {
        let mut engine = MockGraphicsEngine::new(800, 600);
        engine.resize(1920, 1080);

        assert_eq!(engine.width, 1920);
        assert_eq!(engine.height, 1080);
    }

    #[test]
    fn test_test_utils() {
        let state = TestUtils::create_test_app_state();
        assert_eq!(state.frame_count, 0);

        let lesson = TestUtils::create_test_lesson();
        assert_eq!(lesson.id, "test_lesson_1");

        let lessons = TestUtils::create_test_lessons(3);
        assert_eq!(lessons.len(), 3);
        assert_eq!(lessons[0].id, "test_lesson_1");
        assert_eq!(lessons[2].id, "test_lesson_3");
    }

    #[test]
    fn test_performance_test() {
        let mut perf_test = learn_liberty_app::tests::PerformanceTest::new();

        let duration = perf_test.measure(|| {
            std::thread::sleep(Duration::from_millis(10));
        });

        assert!(duration >= Duration::from_millis(10));
        assert_eq!(perf_test.measurements.len(), 1);
        assert!(perf_test.get_total_time() >= Duration::from_millis(10));
    }
}
