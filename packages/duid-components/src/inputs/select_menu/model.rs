use std::collections::{HashSet};
use crate::inputs::button::get_button_default_selectors;

/// Select Menu Model
#[derive(Debug, PartialEq, Clone)]
pub struct SelectMenuModel {
    pub(crate) selectors: HashSet<String>,
    pub(crate) classes: HashSet<String>, // it can use button styles
}



impl SelectMenuModel {
    pub fn new() -> Self {
        let mut classes = HashSet::with_capacity(1);
        classes.insert("btn btn-sm btn-normal btn-default".to_owned());


        SelectMenuModel {
            selectors: get_button_default_selectors(),
            classes
        }
    }

    /// It can use button classes:
    /// default: btn-sm btn-normal btn-default
    /// btn-default, btn-filled, btn-outline, btn-sm , btn-md, btn-lg, btn-normal 
    pub fn add_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.classes.insert(c.as_ref().to_owned());
        });
    }

    pub fn add_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.selectors.insert(c.as_ref().to_owned());
        });
    }

    pub fn remove_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.classes.remove(c.as_ref());
        });
    }

    pub fn remove_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.selectors.remove(c.as_ref());
        });
    }
}