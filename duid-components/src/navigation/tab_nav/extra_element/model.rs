use std::collections::HashSet;

/// ExtraTabNav element Model
#[derive(Debug, PartialEq, Clone)]
pub struct ExtraTabNavElementModel {
    pub(crate) classes: HashSet<String>,
    pub(crate) selectors: HashSet<String>,
    pub(crate) href: String
}

impl ExtraTabNavElementModel {
    pub fn new() -> Self {
        let mut classes = HashSet::with_capacity(0);
        classes.insert("tabnav-extra".to_owned());

        ExtraTabNavElementModel {
            classes,
            selectors: HashSet::with_capacity(0),
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
}
