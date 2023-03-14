
#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    Button,
    Checkbox,
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week
}

impl InputType {
    pub fn to_string(&self) -> String {
        match self {
            InputType::Button => "button".to_owned(),
            InputType::Checkbox => "checkbox".to_owned(),
            InputType::Color => "color".to_owned(),
            InputType::Date => "date".to_owned(),
            InputType::DatetimeLocal => "datetime-local".to_owned(),
            InputType::Email => "email".to_owned(),
            InputType::File => "file".to_owned(),
            InputType::Hidden => "hidden".to_owned(),
            InputType::Image => "image".to_owned(),
            InputType::Month => "month".to_owned(),
            InputType::Number => "number".to_owned(),
            InputType::Password => "password".to_owned(),
            InputType::Radio => "radio".to_owned(),
            InputType::Range => "range".to_owned(),
            InputType::Reset => "reset".to_owned(),
            InputType::Search => "search".to_owned(),
            InputType::Submit => "submit".to_owned(),
            InputType::Tel => "tel".to_owned(),
            InputType::Text => "text".to_owned(),
            InputType::Time => "time".to_owned(),
            InputType::Url => "url".to_owned(),
            InputType::Week => "week".to_owned()
        }
    }
}