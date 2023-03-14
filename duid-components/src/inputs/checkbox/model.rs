use super::InputContext;
use crate::inputs::forms::InputType;
use std::rc::Rc;
use std::cell::RefCell;

/// Input Model
#[derive(Debug, Clone, PartialEq)]
pub struct CheckboxModel {
    pub(crate) input_classes: Vec<String>,
    pub(crate) label_classes: Vec<String>,
    pub(crate) disabled: bool,
    /// The type of the input. 
    pub(crate) type_: InputType,
    pub(crate) input_id: String,
    pub(crate) checked: Rc<RefCell<bool>>,
    pub(crate) label: String,
    pub(crate) note: String,
    pub(crate) show_note: bool,
    pub(crate) context: InputContext,
    pub(crate) emphasized: bool,
    /// Add a custom style, Option<(name, style)>s
    pub(crate) input_style: Option<(&'static str, String)>,
    pub(crate) label_style: Option<(&'static str, String)>
}


impl CheckboxModel {
    pub fn new() -> Self {
        CheckboxModel {
            input_classes: Vec::with_capacity(0),
            label_classes: Vec::with_capacity(0),
            disabled: false,
            /// The type of the input. 
            type_: InputType::Checkbox,
            input_id: String::with_capacity(0),
            checked: Rc::new(RefCell::new(false)),
            label: String::with_capacity(0),
            note: String::with_capacity(0),
            show_note: false,
            context: InputContext::Normal,
            emphasized: false,
            /// Add a custom style, Option<(name, style)>s
            input_style: None,
            label_style: None
        }
    }

    pub fn extend_input_classes(&mut self, classes: &[String]) {
        self.input_classes.extend_from_slice(classes);
        self.input_classes.dedup();
    }
    pub fn extend_label_classes(&mut self, classes: &[String]) {
        self.label_classes.extend_from_slice(classes);
        self.label_classes.dedup();
    }

    pub fn set_disabled(&mut self) {
        self.disabled = !self.disabled;
    }

    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub fn remove_input_classes(&mut self, classes: &[String]) {
        self.input_classes.retain(|c| !classes.contains(c));
        self.input_classes.dedup();
    }

    pub fn remove_label_classes(&mut self, classes: &[String]) {
        self.label_classes.retain(|c| !classes.contains(c));
        self.label_classes.dedup();
    }

    pub fn set_input_id(&mut self, id: &str) {
        self.input_id = id.to_owned();
    }

    pub fn set_checked(&mut self) {
        *self.checked.borrow_mut() = !(*self.checked.borrow());
    }

    pub fn is_checked(&mut self) -> bool {
        *self.checked.borrow()
    }

    pub fn set_label(&mut self, label: &str) {
        self.label = label.to_owned();
    }

    pub fn set_show_note(&mut self) {
        self.show_note = !self.show_note;
    }

    pub fn set_normal_context(&mut self) {
        self.context = InputContext::Normal;
    }

    pub fn set_form_context(&mut self) {
        self.context = InputContext::Form;
    }

    pub fn set_emphasized(&mut self) {
        self.emphasized = !self.emphasized;
    }

    pub fn is_emphasized(&mut self) -> bool {
        self.emphasized
    }

    pub fn set_input_style(&mut self, style: Option<(&'static str, String)>) {
        self.input_style = style;
    }

    pub fn set_label_style(&mut self, style: Option<(&'static str, String)>) {
        self.label_style = style;
    }
}
