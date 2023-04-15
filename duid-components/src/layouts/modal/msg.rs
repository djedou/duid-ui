/// Select Messages
#[derive(Debug, PartialEq, Clone)]
pub enum ModalMsg<M: Clone> {
    Msg(M),
    NoAction
}
