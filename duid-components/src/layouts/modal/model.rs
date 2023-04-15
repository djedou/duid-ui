use std::collections::{HashSet};
use super::default_modal_selectors;
use crate::{
    inputs::buttons::ButtonModel,
    typography::text::TextModel,
};


/// Select Menu Model
#[derive(Debug, PartialEq, Clone)]
pub struct ModalModel {
    pub(crate) selectors: HashSet<String>,
    pub(crate) classes: HashSet<String>,
    pub(crate) is_closed: bool
}



impl ModalModel {
    pub fn new() -> Self {
        let mut classes = HashSet::with_capacity(0);
        classes.insert("details-modal-reset-container details-modal-reset".to_owned());
        

        ModalModel {
            selectors: default_modal_selectors(),
            classes,
            is_closed: true
        }
    }

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

    pub fn closed(&mut self) {
        self.is_closed = true;
    }
    
    pub fn is_closed(&mut self) -> bool {
        self.is_closed
    }
}