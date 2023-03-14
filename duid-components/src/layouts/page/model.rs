use std::collections::HashSet;

/// Page Model
#[derive(Debug, PartialEq, Clone)]
pub struct PageModel {
    pub(crate) classes: HashSet<String>,
    pub(crate) selectors: HashSet<String>,

    pub(crate) header_classes: HashSet<String>,
    pub(crate) header_selectors: HashSet<String>,
    pub(crate) header_height: String,

    pub(crate) body_classes: HashSet<String>,
    pub(crate) body_selectors: HashSet<String>,

    pub(crate) footer_classes: HashSet<String>,
    pub(crate) footer_selectors: HashSet<String>,
    pub(crate) footer_height: String
}

impl PageModel {
    pub fn new() -> PageModel {
        PageModel {
            classes: HashSet::with_capacity(0),
            selectors: HashSet::with_capacity(0),
            header_classes: HashSet::with_capacity(0),
            header_selectors: HashSet::with_capacity(0),
            body_classes: HashSet::with_capacity(0),
            body_selectors: HashSet::with_capacity(0),
            footer_classes: HashSet::with_capacity(0),
            footer_selectors: HashSet::with_capacity(0),
            header_height: "2.25rem".to_owned(),
            footer_height: "6rem".to_owned()
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

    // header
    pub fn add_header_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.header_classes.insert(c.as_ref().to_owned());
        });
    }

    pub fn add_header_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.header_selectors.insert(c.as_ref().to_owned());
        });
    }

    pub fn remove_header_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.header_classes.remove(c.as_ref());
        });
    }

    pub fn remove_header_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.header_selectors.remove(c.as_ref());
        });
    }

    // body
    pub fn add_body_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.body_classes.insert(c.as_ref().to_owned());
        });
    }

    pub fn add_body_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.body_selectors.insert(c.as_ref().to_owned());
        });
    }

    pub fn remove_body_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.body_classes.remove(c.as_ref());
        });
    }

    pub fn remove_body_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.body_selectors.remove(c.as_ref());
        });
    }

    //footer
    pub fn add_footer_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.footer_classes.insert(c.as_ref().to_owned());
        });
    }

    pub fn add_footer_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.footer_selectors.insert(c.as_ref().to_owned());
        });
    }

    pub fn remove_footer_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.footer_classes.remove(c.as_ref());
        });
    }

    pub fn remove_footer_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.footer_selectors.remove(c.as_ref());
        });
    }

    pub fn set_footer_height(&mut self, height: &str) {
        self.footer_height = height.to_string();
    }

    pub fn set_header_height(&mut self, height: &str) {
        self.header_height = height.to_string();
    }
}
