use crate::inputs::button::ButtonMsg;

/// Button Group Message
#[derive(Debug, PartialEq, Clone)]
pub enum ButtonGroupMsg {
    Group((usize, ButtonMsg))
}