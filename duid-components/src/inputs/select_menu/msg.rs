use crate::inputs::buttons::ButtonMsg;

/// Select Messages
#[derive(Debug, PartialEq, Clone)]
pub enum SelectMenuMsg<M: Clone> {
    Msg(M),
    Button(ButtonMsg),
    NoAction
}
