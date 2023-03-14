use crate::inputs::button::ButtonMsg;


/// Button Messages
#[derive(Debug, PartialEq, Clone)]
pub enum InputButtonMsg {
    InputMsg(()),
    ButtonMsg(ButtonMsg),
}
