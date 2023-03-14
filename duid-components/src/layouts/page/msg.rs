
/// Page Message
#[derive(Debug, PartialEq, Clone)]
pub enum PageMsg<M> 
where 
    M: Clone
{
    Element(M)
}