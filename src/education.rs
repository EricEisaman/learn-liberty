use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EducationalContent {
    pub id: String,
    pub title: String,
    pub description: String,
    pub media: Vec<String>, // References to assets
    pub interactive_elements: Vec<InteractiveElement>,
    pub completion_criteria: CompletionCriteria,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractiveElement {
    pub element_type: ElementType,
    pub position: (f32, f32),
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ElementType {
    Text,
    Image,
    Button,
    Quiz,
    Video,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionCriteria {
    pub required_interactions: u32,
    pub time_spent_minimum: f32, // in seconds
    pub quiz_score_threshold: Option<f32>,
}

#[allow(dead_code)]
impl EducationalContent {
    pub fn new(id: String, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            media: Vec::new(),
            interactive_elements: Vec::new(),
            completion_criteria: CompletionCriteria {
                required_interactions: 0,
                time_spent_minimum: 0.0,
                quiz_score_threshold: None,
            },
        }
    }

    pub fn add_media(&mut self, media_path: String) {
        self.media.push(media_path);
    }

    pub fn add_interactive_element(&mut self, element: InteractiveElement) {
        self.interactive_elements.push(element);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_educational_content_creation() {
        let content = EducationalContent::new(
            "test_lesson".to_string(),
            "Test Lesson".to_string(),
            "A test lesson for learning".to_string(),
        );

        assert_eq!(content.id, "test_lesson");
        assert_eq!(content.title, "Test Lesson");
        assert_eq!(content.description, "A test lesson for learning");
        assert!(content.media.is_empty());
        assert!(content.interactive_elements.is_empty());
        assert_eq!(content.completion_criteria.required_interactions, 0);
        assert_eq!(content.completion_criteria.time_spent_minimum, 0.0);
        assert!(content.completion_criteria.quiz_score_threshold.is_none());
    }

    #[test]
    fn test_add_media() {
        let mut content = EducationalContent::new(
            "test_lesson".to_string(),
            "Test Lesson".to_string(),
            "A test lesson".to_string(),
        );

        content.add_media("image1.png".to_string());
        content.add_media("video1.mp4".to_string());

        assert_eq!(content.media.len(), 2);
        assert_eq!(content.media[0], "image1.png");
        assert_eq!(content.media[1], "video1.mp4");
    }

    #[test]
    fn test_add_interactive_element() {
        let mut content = EducationalContent::new(
            "test_lesson".to_string(),
            "Test Lesson".to_string(),
            "A test lesson".to_string(),
        );

        let element = InteractiveElement {
            element_type: ElementType::Button,
            position: (100.0, 200.0),
            data: "click_me".to_string(),
        };

        content.add_interactive_element(element);

        assert_eq!(content.interactive_elements.len(), 1);
        assert_eq!(content.interactive_elements[0].position, (100.0, 200.0));
        assert_eq!(content.interactive_elements[0].data, "click_me");
    }

    #[test]
    fn test_completion_criteria() {
        let criteria = CompletionCriteria {
            required_interactions: 5,
            time_spent_minimum: 120.0,
            quiz_score_threshold: Some(0.8),
        };

        assert_eq!(criteria.required_interactions, 5);
        assert_eq!(criteria.time_spent_minimum, 120.0);
        assert_eq!(criteria.quiz_score_threshold, Some(0.8));
    }
}
