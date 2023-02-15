use std::collections::HashSet;

/// TabNav element Model
#[derive(Debug, PartialEq, Clone)]
pub struct TabNavElementModel {
    pub classes: HashSet<String>,
    pub selectors: HashSet<String>,
    pub selected: bool,
    pub href: String
}

impl TabNavElementModel {
    
    pub fn new() -> Self {
        let mut classes = HashSet::with_capacity(0);
        classes.insert("tabnav-tab".to_owned());
        
        TabNavElementModel {
            classes,
            selectors: HashSet::with_capacity(0),
            selected: false,
            href: String::from("#")
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

    pub fn set_href(&mut self, href: &str) {
        self.href = href.to_owned();
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn is_selected(&mut self) -> bool {
        self.selected
    }
}
