use super::{TabNavItemMsg, ExtraTabNavItemMsg};


/// TabNav Message
#[derive(Debug, PartialEq, Clone)]
pub enum TabNavMsg {
    Item((usize, TabNavItemMsg)),
    ExtraItem((usize, ExtraTabNavItemMsg))
}