


/// The size of the button. Default **Medium**  
#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Size {
    pub fn to_string(&self) -> String {
        match &self {
            Size::Small => "small".to_owned(),
            Size::Medium => "medium".to_owned(),
            Size::Large => "large".to_owned()
        }
    }
}