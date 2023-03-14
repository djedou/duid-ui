
use super::{InputValidation, Size};


/*
pub(crate) fn get_input_type_style(_type_: &InputType, _classes: &mut [String]) {
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
*/

pub(crate) fn get_form_input_size_style(size: &Size, classes: &mut Vec<String>) {
    match size {
        Size::Small => classes.push("FormControl-small".to_owned()),
        Size::Medium => classes.push("FormControl-medium".to_owned()),
        Size::Large => classes.push("FormControl-large".to_owned()),
    }
}

pub(crate) fn get_form_input_validation_style(validation: &InputValidation, classes: &mut Vec<String>) {
    match validation {
        InputValidation::Successed => {
            classes.push("FormControl-success".to_owned());
        },
        InputValidation::Errored => {
            classes.push("FormControl-error".to_owned());
        },
        InputValidation::Warn => {
            classes.push("FormControl-warning".to_owned());
        },
        InputValidation::None => {},
    }
}
