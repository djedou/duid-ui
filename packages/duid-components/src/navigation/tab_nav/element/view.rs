use crate::navigation::tab_nav::{
    TabNavElementModel,
    TabNavItemMsg
};

use duid::{
    html::{
        a,
        attributes::{classes, selectors, empty_attr, Attribute, AttributeValue, Value},
        nodes::Node
    },
    events,
};


/// TabNav Element View
pub fn tab_nav_element_view(tab_nav_model: &TabNavElementModel, children: &[Node<TabNavItemMsg>]) -> Node<TabNavItemMsg> {
    let mut new_classes = vec![];
    
    if tab_nav_model.selected {
        new_classes.push("selected".to_owned());
    }

    let classes_vec: Vec<_> = tab_nav_model.classes.iter().map(|c| c.to_owned()).collect(); 
    let selectors_vec: Vec<_> = tab_nav_model.selectors.iter().collect();   
    new_classes.extend_from_slice(&classes_vec);
    
    a(
        &[
            events::on_click(|_| TabNavItemMsg::OnClick),
            if tab_nav_model.selected {
                Attribute::new(None, "aria-current", AttributeValue::from_value(Value::String("page".to_owned())))
            }
            else {
                empty_attr()
            },
            Attribute::new(None, "href", AttributeValue::from_value(Value::String(tab_nav_model.href.clone()))),
            classes(&new_classes),
            selectors(&selectors_vec)
        ],
        children
    )
}
