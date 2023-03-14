use std::collections::HashSet;
use crate::navigation::tab_nav::get_tab_nav_selectors;

/// TabNav Model
#[derive(Debug, PartialEq, Clone)]
pub struct TabNavModel {
    pub(crate) classes: HashSet<String>,
    pub(crate) selectors: HashSet<String>,
    pub(crate) nav_classes: HashSet<String>,
    pub(crate) nav_selectors: HashSet<String>,
    pub(crate) extra_nav_classes: HashSet<String>,
    pub(crate) extra_nav_selectors: HashSet<String>,

    pub(crate) aria_label: String,
}


impl TabNavModel {
    pub fn new() -> Self {
        let mut classes = HashSet::with_capacity(0);
        let _ = classes.insert("tabnav".to_owned());

        TabNavModel {
            classes,
            selectors: get_tab_nav_selectors(),
            nav_classes: HashSet::with_capacity(0),
            nav_selectors: HashSet::with_capacity(0),
            extra_nav_classes: HashSet::with_capacity(0),
            extra_nav_selectors: HashSet::with_capacity(0),

            aria_label: String::with_capacity(0)
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

    // nav
    pub fn add_nav_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.nav_classes.insert(c.as_ref().to_owned());
        });
    }

    pub fn add_nav_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.nav_selectors.insert(c.as_ref().to_owned());
        });
    }

    pub fn remove_nav_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.nav_classes.remove(c.as_ref());
        });
    }

    pub fn remove_nav_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.nav_selectors.remove(c.as_ref());
        });
    }

    // extra nav
    pub fn add_extra_nav_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.extra_nav_classes.insert(c.as_ref().to_owned());
        });
    }

    pub fn add_extra_nav_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.extra_nav_selectors.insert(c.as_ref().to_owned());
        });
    }

    pub fn remove_extra_nav_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.extra_nav_classes.remove(c.as_ref());
        });
    }

    pub fn remove_extra_nav_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.extra_nav_selectors.remove(c.as_ref());
        });
    }

    pub fn set_aria_label(&mut self, aria_label: &str) {
        self.aria_label = aria_label.to_owned();
    }
}

