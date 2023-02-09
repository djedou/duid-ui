
/// Button Group Model
#[derive(Debug, Clone, PartialEq)]
pub struct ButtonGroupModel {
    pub classes: Vec<String>,
    /// Add a custom style, Option<(name, style)>s
    pub style: Option<(&'static str, String)>
}

impl ButtonGroupModel {
    pub fn new() -> Self {
        ButtonGroupModel {
            classes: Vec::with_capacity(0),
            style: None
        }
    }

    pub fn extend_classes(&mut self, classes: &[String]) {
        self.classes.extend_from_slice(classes);
        self.classes.dedup();
    }

    pub fn remove_classes(&mut self, classes: &[String]) {
        self.classes.retain(|c| !classes.contains(c));
        self.classes.dedup();
    }

    pub fn set_style(&mut self, style: Option<(&'static str, String)>) {
        self.style = style;
    }
}