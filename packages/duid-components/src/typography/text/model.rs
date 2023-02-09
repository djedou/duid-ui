use std::collections::HashSet;
use super::{TextTag, TextMsg};
use duid::html::attributes::{Attribute, AttributeValue};

/// Text Model
#[derive(Debug, PartialEq, Clone)]
pub struct TextModel {
    pub(crate) classes: HashSet<String>,
    pub(crate) selectors: HashSet<String>,
    pub(crate) attributes: Vec<Attribute<TextMsg>>,
    pub(crate) tag: TextTag,
}

impl TextModel {
    pub fn new() -> TextModel {
        TextModel {
            classes: HashSet::with_capacity(0),
            selectors: HashSet::with_capacity(0),
            attributes: Vec::with_capacity(0),
            tag: TextTag::Span
        }
    }

    pub fn add_attributes(&mut self, attributes: &[Attribute<TextMsg>]) {
        self.attributes.extend_from_slice(attributes);
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

    pub fn set_tag_div(&mut self) {
        self.tag = TextTag::Div;
    }

    pub fn set_tag_span(&mut self) {
        self.tag = TextTag::Span;
    }

    pub fn set_tag_italic(&mut self) {
        self.tag = TextTag::Italic;
    }

    pub fn set_tag_em(&mut self) {
        self.tag = TextTag::Em;
    }

    pub fn set_tag_strong(&mut self) {
        self.tag = TextTag::Strong;
    }

    pub fn set_tag_mark(&mut self) {
        self.tag = TextTag::Mark;
    }

    pub fn set_tag_cite(&mut self) {
        self.tag = TextTag::Cite;
    }

    pub fn set_tag_dfn(&mut self) {
        self.tag = TextTag::Dfn;
    }
}