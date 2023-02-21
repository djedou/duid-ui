
/// Select Messages
#[derive(Debug, PartialEq, Clone)]
pub enum SelectMenuMsg<M: Clone> {
    Msg(M),
    NoAction
}
