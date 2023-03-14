
#[derive(Debug, Clone, PartialEq)]
pub enum InputContext {
    Normal,
    Form
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputValidation {
    Successed,
    Warn,
    Errored,
    None
}


#[derive(Debug, Clone, PartialEq)]
pub enum InputVariant {
    Default,
    Label,
    FloatLabel,
    //AddOnAppended,
    //AddOnPrepended,
    //AppendedPprependedAddOns
}


#[derive(Debug, Clone, PartialEq)]
pub struct InputValidationNote {
    pub(crate) successed: String,
    pub(crate) warn: String,
    pub(crate) errored: String,
    pub(crate) none: String
}

impl InputValidationNote {
    pub(crate) fn new() -> Self {
        InputValidationNote {
            successed: String::with_capacity(0),
            warn: String::with_capacity(0),
            errored: String::with_capacity(0),
            none: String::with_capacity(0)
        }
    }

    pub(crate) fn set_successed(&mut self, successed: &str) {
        self.successed = successed.to_owned();
    }

    pub(crate) fn set_warn(&mut self, warn: &str) {
        self.warn = warn.to_owned();
    }

    pub(crate) fn set_errored(&mut self, errored: &str) {
        self.errored = errored.to_owned();
    }
}