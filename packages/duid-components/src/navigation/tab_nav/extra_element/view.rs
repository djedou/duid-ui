use crate::navigation::tab_nav::{
    ExtraTabNavElementModel,
    ExtraTabNavItemMsg
};

use duid::{
    html::{
        a,
        attributes::{classes, selectors, Attribute, AttributeValue, Value},
        nodes::Node
    },
    events
};


/// ExtraTabNav Element View
pub fn extra_tab_nav_element_view(tab_nav_model: &ExtraTabNavElementModel, children: &[Node<ExtraTabNavItemMsg>]) -> Node<ExtraTabNavItemMsg> {

    let classes_vec: Vec<_> = tab_nav_model.classes.iter().map(|c| c.to_owned()).collect(); 
    let selectors_vec: Vec<_> = tab_nav_model.selectors.iter().collect();
    
    a(
        &[
            events::on_click(|_| ExtraTabNavItemMsg::OnClick),
            Attribute::new(None, "href", AttributeValue::from_value(Value::String(tab_nav_model.href.clone()))),
            classes(&classes_vec),
            selectors(&selectors_vec)
        ],
        children
    )
}
