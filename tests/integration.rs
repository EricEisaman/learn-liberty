//! Integration tests for Learn Liberty
//! 
//! These tests verify that different components work together correctly
//! and that the overall system behaves as expected.

use learn_liberty_app::{state::AppState, education::EducationalContent};
use learn_liberty_app::tests::{MockWindow, MockGraphicsEngine, TestUtils, PerformanceTest};

/// Test the interaction between app state and educational content
#[test]
fn test_app_state_with_educational_content() {
    let mut state = AppState::default();
    let mut lesson = TestUtils::create_test_lesson();
    
    // Add some interactive elements to the lesson
    lesson.add_media("test_image.png".to_string());
    lesson.add_media("test_video.mp4".to_string());
    
    // Advance the lesson in the app state
    state.advance_lesson(lesson.id.clone(), 0.5);
    
    // Verify the state reflects the lesson progress
    assert_eq!(state.current_lesson_id, lesson.id);
    assert_eq!(state.lesson_progress, 0.5);
    
    // Update the state to simulate time passing
    state.update(0.016); // 16ms frame time
    assert_eq!(state.frame_count, 1);
    assert_eq!(state.time, 0.016);
}

/// Test the interaction between window and graphics engine
#[test]
fn test_window_graphics_integration() {
    let (window, event_sender) = MockWindow::new("Test Window", 800, 600);
    let mut graphics = MockGraphicsEngine::new(800, 600);
    
    // Verify initial state
    assert_eq!(window.get_size(), (800, 600));
    assert_eq!(graphics.width, 800);
    assert_eq!(graphics.height, 600);
    
    // Simulate a resize event
    event_sender.send(learn_liberty_app::tests::MockEvent::Resize { 
        width: 1920, 
        height: 1080 
    }).unwrap();
    
    // Handle the resize in graphics engine
    graphics.resize(1920, 1080);
    
    // Verify the resize was applied
    assert_eq!(graphics.width, 1920);
    assert_eq!(graphics.height, 1080);
    
    // Test rendering
    let render_result = graphics.render();
    assert!(render_result.is_ok());
    assert_eq!(graphics.get_render_calls(), 1);
}

/// Test the complete application loop simulation
#[test]
fn test_application_loop_simulation() {
    let mut state = AppState::default();
    let mut graphics = MockGraphicsEngine::new(1024, 768);
    let (_window, _event_sender) = MockWindow::new("Learn Liberty", 1024, 768);
    
    // Simulate 60 frames (1 second at 60 FPS)
    let frame_time = 1.0 / 60.0; // ~16.67ms
    
    for frame in 1..=60 {
        // Update application state
        state.update(frame_time);
        
        // Render frame
        let render_result = graphics.render();
        assert!(render_result.is_ok());
        
        // Verify frame count matches
        assert_eq!(state.frame_count, frame);
        assert_eq!(graphics.get_frame_count(), frame);
    }
    
    // Verify final state
    assert_eq!(state.frame_count, 60);
    assert_eq!(graphics.get_render_calls(), 60);
    assert!((state.time - 1.0).abs() < 0.01); // Should be close to 1 second
}

/// Test educational content progression
#[test]
fn test_educational_content_progression() {
    let mut state = AppState::default();
    let lessons = TestUtils::create_test_lessons(3);
    
    // Progress through lessons
    for (i, lesson) in lessons.iter().enumerate() {
        let progress = (i as f32 + 1.0) / lessons.len() as f32;
        state.advance_lesson(lesson.id.clone(), progress);
        
        assert_eq!(state.current_lesson_id, lesson.id);
        assert_eq!(state.lesson_progress, progress);
    }
    
    // Verify final lesson state
    assert_eq!(state.current_lesson_id, "test_lesson_3");
    assert_eq!(state.lesson_progress, 1.0);
}

/// Test performance characteristics
#[test]
fn test_performance_characteristics() {
    let mut perf_test = PerformanceTest::new();
    let mut state = AppState::default();
    let mut graphics = MockGraphicsEngine::new(800, 600);
    
    // Measure state update performance
    let update_time = perf_test.measure(|| {
        for _ in 0..1000 {
            state.update(0.016);
        }
    });
    
    // Measure rendering performance
    let render_time = perf_test.measure(|| {
        for _ in 0..100 {
            graphics.render().unwrap();
        }
    });
    
    // Verify performance is reasonable
    assert!(update_time < std::time::Duration::from_millis(100));
    assert!(render_time < std::time::Duration::from_millis(1000));
    
    // Verify measurements were recorded
    assert_eq!(perf_test.measurements.len(), 2);
    assert!(perf_test.get_average_time() > std::time::Duration::from_secs(0));
}

/// Test error handling and recovery
#[test]
fn test_error_handling() {
    let mut graphics = MockGraphicsEngine::new(800, 600);
    
    // Test normal operation
    let result = graphics.render();
    assert!(result.is_ok());
    
    // Test multiple renders
    for _ in 0..10 {
        let result = graphics.render();
        assert!(result.is_ok());
    }
    
    assert_eq!(graphics.get_render_calls(), 11);
}

/// Test concurrent operations simulation
#[test]
fn test_concurrent_operations_simulation() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    let state = Arc::new(Mutex::new(AppState::default()));
    let graphics = Arc::new(Mutex::new(MockGraphicsEngine::new(800, 600)));
    
    let state_clone = Arc::clone(&state);
    let graphics_clone = Arc::clone(&graphics);
    
    // Simulate concurrent state updates and rendering
    let handle = thread::spawn(move || {
        for _ in 0..30 {
            if let Ok(mut state) = state_clone.lock() {
                state.update(0.016);
            }
            if let Ok(mut graphics) = graphics_clone.lock() {
                graphics.render().unwrap();
            }
        }
    });
    
    handle.join().unwrap();
    
    // Verify final state
    let final_state = state.lock().unwrap();
    let final_graphics = graphics.lock().unwrap();
    
    assert_eq!(final_state.frame_count, 30);
    assert_eq!(final_graphics.get_render_calls(), 30);
}

/// Test memory usage and resource management
#[test]
fn test_memory_usage() {
    let mut lessons = Vec::new();
    
    // Create many lessons to test memory usage
    for i in 0..1000 {
        let mut lesson = EducationalContent::new(
            format!("lesson_{}", i),
            format!("Lesson {}", i),
            format!("Description for lesson {}", i),
        );
        
        // Add some content to each lesson
        lesson.add_media(format!("image_{}.png", i));
        lesson.add_media(format!("video_{}.mp4", i));
        
        lessons.push(lesson);
    }
    
    // Verify all lessons were created
    assert_eq!(lessons.len(), 1000);
    
    // Verify lesson content
    assert_eq!(lessons[0].id, "lesson_0");
    assert_eq!(lessons[999].id, "lesson_999");
    assert_eq!(lessons[0].media.len(), 2);
    assert_eq!(lessons[999].media.len(), 2);
}
