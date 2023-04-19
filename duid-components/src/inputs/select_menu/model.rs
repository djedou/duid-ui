use std::collections::{HashSet};
use super::default_select_menu_selectors;
use crate::{
    inputs::buttons::ButtonModel,
    typography::text::TextModel,
};


/// Select Menu Model
#[derive(Debug, PartialEq, Clone)]
pub struct SelectMenuModel {
    pub(crate) selectors: HashSet<String>,
    pub(crate) classes: HashSet<String>,
    pub button_model: ButtonModel,
    pub button_text: String,
    pub button_text_model: TextModel,
    pub(crate) right_aligned: bool,
    pub(crate) closed: bool
}



impl SelectMenuModel {
    pub fn new() -> Self {
        let mut button_model = ButtonModel::new();
        button_model.set_size_sm();
        button_model.set_kind_default();
        button_model.set_colors_default();
        button_model.set_variation_summary();

        let mut classes = HashSet::with_capacity(0);
        classes.insert("select-menu-container details-reset".to_owned());
        

        SelectMenuModel {
            selectors: default_select_menu_selectors(),
            classes,
            button_model,
            button_text: String::from("Select"),
            button_text_model: TextModel::new(),
            right_aligned: false,
            closed: true
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

    pub fn set_button_text(&mut self, button_text: impl AsRef<str>) {
        self.button_text = button_text.as_ref().to_string();
    }

    pub fn set_right_aligned(&mut self, open: bool) {
        self.right_aligned = open;
    }
    
    pub fn is_right_aligned(&self) -> bool {
        self.right_aligned
    }

    pub fn set_closed(&mut self, open: bool) {
        self.closed = open;
    }
    
    pub fn is_closed(&self) -> bool {
        self.closed
    }
}