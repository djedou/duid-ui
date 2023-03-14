/// Sidebar Message
#[derive(Debug, PartialEq, Clone)]
pub enum SidebarMsg<M> 
where 
    M: Clone
{
    Element(M)
}