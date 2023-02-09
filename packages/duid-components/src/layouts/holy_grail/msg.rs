
/// HolyGrail Message
#[derive(Debug, PartialEq, Clone)]
pub enum HolyGrailMsg<M> 
where 
    M: Clone
{
    Element(M)
}