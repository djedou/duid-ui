use crate::inputs::buttons::ButtonMsg;

/// Select Messages
#[derive(Debug, PartialEq, Clone)]
pub enum ModalMsg<M: Clone> {
    Msg(M),
    Button(ButtonMsg),
    NoAction
}
