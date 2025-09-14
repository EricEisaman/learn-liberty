#[derive(Debug)]
pub struct AppState {
    pub frame_count: u32,
    pub time: f64,
    pub lesson_progress: f32,
    pub current_lesson_id: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            frame_count: 0,
            time: 0.0,
            lesson_progress: 0.0,
            current_lesson_id: String::new(),
        }
    }
}

impl AppState {
    pub fn update(&mut self, delta_time: f64) {
        self.frame_count += 1;
        self.time += delta_time;
    }

    pub fn advance_lesson(&mut self, lesson_id: String, progress: f32) {
        self.current_lesson_id = lesson_id;
        self.lesson_progress = progress;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_default() {
        let state = AppState::default();
        
        assert_eq!(state.frame_count, 0);
        assert_eq!(state.time, 0.0);
        assert_eq!(state.lesson_progress, 0.0);
        assert_eq!(state.current_lesson_id, String::new());
    }

    #[test]
    fn test_app_state_update() {
        let mut state = AppState::default();
        let initial_frame_count = state.frame_count;
        let initial_time = state.time;
        
        state.update(0.016); // Update with 16ms (60 FPS)
        
        assert_eq!(state.frame_count, initial_frame_count + 1);
        assert_eq!(state.time, initial_time + 0.016);
    }

    #[test]
    fn test_app_state_advance_lesson() {
        let mut state = AppState::default();
        let lesson_id = "lesson_1".to_string();
        let progress = 0.5;
        
        state.advance_lesson(lesson_id.clone(), progress);
        
        assert_eq!(state.current_lesson_id, lesson_id);
        assert_eq!(state.lesson_progress, progress);
    }

    #[test]
    fn test_app_state_multiple_updates() {
        let mut state = AppState::default();
        
        for i in 1..=10 {
            state.update(0.016);
            assert_eq!(state.frame_count, i);
            assert!((state.time - (i as f64) * 0.016).abs() < 1e-10);
        }
    }
}
