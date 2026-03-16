#[derive(Default, Clone)]
pub struct Status {
    pub text: String,
    pub color: egui::Color32,
}

impl Status {
    pub fn new(text: impl Into<String>, color: egui::Color32) -> Self {
        Self {
            text: text.into(),
            color,
        }
    }
    pub fn success(text: impl Into<String>) -> Self {
        Self::new(text, egui::Color32::LIGHT_GREEN)
    }

    pub fn info(text: impl Into<String>) -> Self {
        Self::new(text, egui::Color32::LIGHT_BLUE)
    }

    pub fn warning(text: impl Into<String>) -> Self {
        Self::new(text, egui::Color32::ORANGE)
    }

    pub fn error(text: impl Into<String>) -> Self {
        Self::new(text, egui::Color32::RED)
    }

    pub fn neutral(text: impl Into<String>) -> Self {
        Self::new(text, egui::Color32::GRAY)
    }

    pub fn clear(&mut self) {
        self.text.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}
