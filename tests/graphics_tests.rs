//! Graphics-specific tests for Learn Liberty
//! 
//! These tests focus on graphics rendering, performance,
//! and visual output validation.

use learn_liberty_app::tests::{MockGraphicsEngine, PerformanceTest};
use std::time::Duration;

/// Test basic graphics engine functionality
#[test]
fn test_graphics_engine_basic_operations() {
    let mut engine = MockGraphicsEngine::new(1920, 1080);
    
    // Test initial state
    assert_eq!(engine.width, 1920);
    assert_eq!(engine.height, 1080);
    assert_eq!(engine.get_frame_count(), 0);
    assert_eq!(engine.get_render_calls(), 0);
    
    // Test rendering
    let result = engine.render();
    assert!(result.is_ok());
    assert_eq!(engine.get_frame_count(), 1);
    assert_eq!(engine.get_render_calls(), 1);
    
    // Test multiple renders
    for i in 2..=10 {
        let result = engine.render();
        assert!(result.is_ok());
        assert_eq!(engine.get_frame_count(), i);
        assert_eq!(engine.get_render_calls(), i);
    }
}

/// Test graphics engine resize operations
#[test]
fn test_graphics_engine_resize() {
    let mut engine = MockGraphicsEngine::new(800, 600);
    
    // Test initial size
    assert_eq!(engine.width, 800);
    assert_eq!(engine.height, 600);
    
    // Test resize to different aspect ratios
    let test_sizes = vec![
        (1024, 768),   // 4:3
        (1920, 1080),  // 16:9
        (2560, 1440),  // 16:9 (2K)
        (3840, 2160),  // 16:9 (4K)
        (800, 600),    // Back to original
    ];
    
    for (width, height) in test_sizes {
        engine.resize(width, height);
        assert_eq!(engine.width, width);
        assert_eq!(engine.height, height);
        
        // Verify rendering still works after resize
        let result = engine.render();
        assert!(result.is_ok());
    }
}

/// Test graphics performance benchmarks
#[test]
fn test_graphics_performance_benchmarks() {
    let mut perf_test = PerformanceTest::new();
    let mut engine = MockGraphicsEngine::new(1920, 1080);
    
    // Benchmark single frame rendering
    let single_frame_time = perf_test.measure(|| {
        engine.render().unwrap();
    });
    
    // Benchmark multiple frame rendering
    let multi_frame_time = perf_test.measure(|| {
        for _ in 0..100 {
            engine.render().unwrap();
        }
    });
    
    // Verify performance is reasonable (very lenient for CI)
    assert!(single_frame_time < Duration::from_millis(100));
    assert!(multi_frame_time < Duration::from_millis(2000));
    
    // Verify frame counts
    assert_eq!(engine.get_render_calls(), 101); // 1 + 100
    assert_eq!(engine.get_frame_count(), 101);
}

/// Test graphics engine with different resolutions
#[test]
fn test_graphics_engine_resolutions() {
    let resolutions = vec![
        (640, 480),    // VGA
        (800, 600),    // SVGA
        (1024, 768),   // XGA
        (1280, 720),   // HD
        (1920, 1080),  // Full HD
        (2560, 1440),  // 2K
        (3840, 2160),  // 4K
    ];
    
    for (width, height) in resolutions {
        let mut engine = MockGraphicsEngine::new(width, height);
        
        // Test initial state
        assert_eq!(engine.width, width);
        assert_eq!(engine.height, height);
        
        // Test rendering at this resolution
        let result = engine.render();
        assert!(result.is_ok());
        
        // Test multiple renders
        for _ in 0..5 {
            let result = engine.render();
            assert!(result.is_ok());
        }
        
        assert_eq!(engine.get_render_calls(), 6);
    }
}

/// Test graphics engine stress testing
#[test]
fn test_graphics_engine_stress() {
    let mut engine = MockGraphicsEngine::new(1920, 1080);
    let mut perf_test = PerformanceTest::new();
    
    // Stress test with many rapid renders
    let stress_time = perf_test.measure(|| {
        for _ in 0..1000 {
            engine.render().unwrap();
        }
    });
    
    // Verify all renders completed successfully
    assert_eq!(engine.get_render_calls(), 1000);
    assert_eq!(engine.get_frame_count(), 1000);
    
    // Verify performance is still reasonable (very lenient for CI)
    assert!(stress_time < Duration::from_millis(10000));
    
    // Test average render time (very lenient for CI)
    let avg_time = stress_time / 1000;
    assert!(avg_time < Duration::from_millis(10));
}

/// Test graphics engine with rapid resize operations
#[test]
fn test_graphics_engine_rapid_resize() {
    let mut engine = MockGraphicsEngine::new(800, 600);
    
    // Rapid resize operations
    let resize_sequence = vec![
        (1024, 768),
        (1920, 1080),
        (800, 600),
        (1280, 720),
        (1920, 1080),
        (800, 600),
    ];
    
    for (width, height) in &resize_sequence {
        engine.resize(*width, *height);
        assert_eq!(engine.width, *width);
        assert_eq!(engine.height, *height);
        
        // Render after each resize
        let result = engine.render();
        assert!(result.is_ok());
    }
    
    // Verify final state
    assert_eq!(engine.width, 800);
    assert_eq!(engine.height, 600);
    assert_eq!(engine.get_render_calls(), resize_sequence.len() as u32);
}

/// Test graphics engine memory usage
#[test]
fn test_graphics_engine_memory_usage() {
    let mut engines = Vec::new();
    
    // Create multiple graphics engines
    for i in 0..100 {
        let engine = MockGraphicsEngine::new(800 + i, 600 + i);
        engines.push(engine);
    }
    
    // Verify all engines were created
    assert_eq!(engines.len(), 100);
    
    // Test rendering on all engines
    for (i, engine) in engines.iter_mut().enumerate() {
        let result = engine.render();
        assert!(result.is_ok());
        assert_eq!(engine.width, 800 + i as u32);
        assert_eq!(engine.height, 600 + i as u32);
    }
}

/// Test graphics engine error recovery
#[test]
fn test_graphics_engine_error_recovery() {
    let mut engine = MockGraphicsEngine::new(1920, 1080);
    
    // Test normal operation
    let result = engine.render();
    assert!(result.is_ok());
    
    // Test continuous operation
    for _ in 0..50 {
        let result = engine.render();
        assert!(result.is_ok());
    }
    
    // Verify state is consistent
    assert_eq!(engine.get_render_calls(), 51);
    assert_eq!(engine.get_frame_count(), 51);
    
    // Test resize and continue rendering
    engine.resize(800, 600);
    let result = engine.render();
    assert!(result.is_ok());
    
    assert_eq!(engine.width, 800);
    assert_eq!(engine.height, 600);
    assert_eq!(engine.get_render_calls(), 52);
}

/// Test graphics engine with different frame rates
#[test]
fn test_graphics_engine_frame_rates() {
    let frame_rates = vec![30, 60, 120, 144];
    
    for fps in frame_rates {
        let mut engine = MockGraphicsEngine::new(1920, 1080);
        let _frame_time = 1.0 / fps as f64;
        
        // Simulate rendering at this frame rate for 1 second
        let frames = fps;
        for _ in 0..frames {
            let result = engine.render();
            assert!(result.is_ok());
        }
        
        assert_eq!(engine.get_render_calls(), frames);
        assert_eq!(engine.get_frame_count(), frames);
    }
}

/// Test graphics engine performance consistency
#[test]
fn test_graphics_engine_performance_consistency() {
    let mut engine = MockGraphicsEngine::new(1920, 1080);
    let mut perf_test = PerformanceTest::new();
    
    // Measure performance over multiple runs
    for _ in 0..10 {
        let render_time = perf_test.measure(|| {
            for _ in 0..10 {
                engine.render().unwrap();
            }
        });
        
        // Each run should be reasonably consistent (more lenient for CI)
        assert!(render_time < Duration::from_millis(200));
    }
    
    // Verify all renders completed
    assert_eq!(engine.get_render_calls(), 100); // 10 runs * 10 renders each
    
    // Test average performance (very lenient for CI)
    let avg_time = perf_test.get_average_time();
    assert!(avg_time > Duration::from_secs(0));
    assert!(avg_time < Duration::from_millis(200));
}
