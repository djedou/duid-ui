use super::{InputVariant, InputValidation, InputContext, InputValidationNote, Size, InputType, default_input_selectors};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashSet};

/// Input Model
#[derive(Debug, Clone, PartialEq)]
pub struct InputModel {
    pub(crate) selectors: HashSet<String>,
    pub(crate) input_classes: Vec<String>,
    pub(crate) label_classes: Vec<String>,
    /// If `true`, the input is disabled. Default false
    pub(crate) disabled: bool,
    /// The size of the input.
    pub(crate) size: Size,
    /// The type of the input. 
    pub(crate) type_: InputType,
    pub(crate) input_id: String,
    /// The variant of the input. 
    pub(crate) variant: InputVariant,  
    pub(crate) aria_label: String,
    pub(crate) placeholder: String,
    pub(crate) value: Rc<RefCell<String>>,
    pub(crate) label: String,
    pub(crate) context: InputContext,
    pub(crate) input_group_inline: bool,
    pub(crate) validation: InputValidation,
    pub(crate) full_width: bool,
    pub(crate) validation_note: InputValidationNote,
    /// Add a custom style, Option<(name, style)>s
    pub(crate) input_style: Option<(&'static str, String)>,
    pub(crate) label_style: Option<(&'static str, String)>
}


impl InputModel {
    pub fn new() -> Self {
        InputModel {
            selectors: default_input_selectors(),
            input_classes: Vec::with_capacity(0),
            label_classes: Vec::with_capacity(0),
            disabled: false,
            size: Size::Medium,
            type_: InputType::Text, 
            input_id: String::with_capacity(0),
            value: Rc::new(RefCell::new(String::with_capacity(0))),
            variant: InputVariant::FloatLabel,  
            aria_label: String::with_capacity(0),
            label: String::with_capacity(0),
            placeholder: String::with_capacity(0),
            context: InputContext::Normal,
            input_group_inline: false,
            validation: InputValidation::None,
            full_width: false,
            validation_note: InputValidationNote::new(),
            input_style: None,
            label_style: None
        }
    }

    pub fn add_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.selectors.insert(c.as_ref().to_owned());
        });
    }

    pub fn remove_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.selectors.remove(c.as_ref());
        });
    }

    pub fn extend_input_classes(&mut self, classes: &[String]) {
        self.input_classes.extend_from_slice(classes);
        self.input_classes.dedup();
    }

    pub fn extend_label_classes(&mut self, classes: &[String]) {
        self.label_classes.extend_from_slice(classes);
        self.label_classes.dedup();
    }

    pub fn remove_input_classes(&mut self, classes: &[String]) {
        self.input_classes.retain(|c| !classes.contains(c));
        self.input_classes.dedup();
    }

    pub fn remove_label_classes(&mut self, classes: &[String]) {
        self.label_classes.retain(|c| !classes.contains(c));
        self.label_classes.dedup();
    }

    pub fn set_disabled(&mut self) {
        self.disabled = !self.disabled;
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub fn set_aria_label(&mut self, aria_label: &str) {
        self.aria_label = aria_label.to_owned();
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_owned();
    }

    pub fn set_placeholder(&mut self, placeholder: &str) {
        self.placeholder = placeholder.to_owned();
    }

    pub fn set_size_sm(&mut self) {
        self.size = Size::Small;
    }

    pub fn set_size_md(&mut self) {
        self.size = Size::Medium;
    }

    pub fn set_size_large(&mut self) {
        self.size = Size::Large;
    }

    pub fn set_type_text(&mut self) {
        self.type_ = InputType::Text;
    }

    pub fn set_type_email(&mut self) {
        self.type_ = InputType::Email;
    }

    pub fn set_type_password(&mut self) {
        self.type_ = InputType::Password;
    }

    pub fn set_type_number(&mut self) {
        self.type_ = InputType::Number;
    }

    pub fn set_type_tel(&mut self) {
        self.type_ = InputType::Tel;
    }

    pub fn set_type_search(&mut self) {
        self.type_ = InputType::Search;
    }

    pub fn set_input_id(&mut self, id: &str) {
        self.input_id = id.to_owned();
    }

    pub fn set_variant_default(&mut self) {
        self.variant = InputVariant::Default;
    }

    pub fn set_variant_label(&mut self) {
        self.variant = InputVariant::Label;
    }

    pub fn set_variant_float_label(&mut self) {
        self.variant = InputVariant::FloatLabel;
    }

    pub fn set_normal_context(&mut self) {
        self.context = InputContext::Normal;
    }

    pub fn set_form_context(&mut self) {
        self.context = InputContext::Form;
    }

    pub fn set_input_group_inline(&mut self) {
        self.input_group_inline = !self.input_group_inline;
    }

    pub fn set_full_width(&mut self) {
        self.full_width = !self.full_width;
    }

    pub fn set_validation_none(&mut self) {
        self.validation = InputValidation::None;
    }

    pub fn set_validation_successed(&mut self) {
        self.validation = InputValidation::Successed;
    }

    pub fn set_validation_warn(&mut self) {
        self.validation = InputValidation::Warn;
    }

    pub fn set_validation_errored(&mut self) {
        self.validation = InputValidation::Errored;
    }

    pub fn set_validation_successed_note(&mut self, successed: &str) {
        self.validation_note.set_successed(successed);
    }

    pub fn set_validation_warn_note(&mut self, warn: &str) {
        self.validation_note.set_warn(warn);
    }

    pub fn set_validation_errored_note(&mut self, errored: &str) {
        self.validation_note.set_errored(errored);
    }

    pub fn set_input_style(&mut self, style: Option<(&'static str, String)>) {
        self.input_style = style;
    }

    pub fn set_label_style(&mut self, style: Option<(&'static str, String)>) {
        self.label_style = style;
    }
}
