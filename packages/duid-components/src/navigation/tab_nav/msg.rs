use super::{TabNavItemMsg, ExtraTabNavItemMsg};


/// TabNav Message
#[derive(Debug, PartialEq, Clone)]
pub enum TabNavMsg<M: Clone> {
    Item((usize, TabNavItemMsg)),
    ExtraItem((usize, ExtraTabNavItemMsg)),
    Content(M)
}