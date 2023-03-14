
use super::{InputValidation, Size};


pub(crate) fn get_normal_input_size_style(size: &Size, classes: &mut Vec<String>) {
    match size {
        Size::Small => { classes.push("input-sm".to_owned()); },
        Size::Medium => { classes.push("input-lg".to_owned()); },
        Size::Large => { classes.push("input-block".to_owned()); },
    }
}

pub(crate) fn get_normal_group_style(validation: &InputValidation, is_flat: bool) -> Vec<String> {
    
    let mut classes = 
        if is_flat {
            vec!["form-group".to_owned(), "flattened".to_owned()]
        } 
        else {
            vec!["form-group".to_owned()]
        };
    
    match validation {
        InputValidation::Successed => classes.push("successed".to_owned()),
        InputValidation::Warn => classes.push("warn".to_owned()),
        InputValidation::Errored => classes.push("errored".to_owned()),
        InputValidation::None => {}
    }

    classes
}

pub(crate) fn get_normal_group_note_style(validation: &InputValidation) -> Vec<String> {
    
    let mut classes = vec!["note".to_owned()];
    
    match validation {
        InputValidation::Successed => classes.push("success".to_owned()),
        InputValidation::Warn => classes.push("warning".to_owned()),
        InputValidation::Errored => classes.push("error".to_owned()),
        InputValidation::None => {}
    }

    classes
}
