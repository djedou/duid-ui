use duid::{
    html::{
        div, 
        attributes::{style, classes, empty_attr},
        nodes::Node
    },
    duid_events::NodeMapMsg
};
use crate::inputs::buttons::{ButtonMsg};
use super::{ButtonGroupModel, ButtonGroupMsg};


/// Group Button View
pub fn button_group_view(group_model: &ButtonGroupModel, children: Vec<(usize, Node<ButtonMsg>)>) -> Node<ButtonGroupMsg> {
    
    let mut new_classes = vec!["BtnGroup".to_owned()];
    new_classes.extend_from_slice(&group_model.classes);

    let children_view: Vec<_> = 
        children.iter()
        .map(|(index, child)| {
            let i = index.clone();
            child.to_owned().map_msg(move |btn_grp_msg| ButtonGroupMsg::Group((i, btn_grp_msg)))
        })
        .collect();
    
    div(
        &[
            classes(&new_classes),
            if let Some((name, s_value)) = &group_model.style {
                style(name, s_value)
            }
            else {
                empty_attr()
            },
            style("duid-buttons-styles", String::with_capacity(0)),
        ],
        &children_view
    )
}
