use super::{TabNavModel, TabNavItemMsg, TabNavMsg, ExtraTabNavItemMsg};
use duid::{
    html::{
        nav, div,
        attributes::{classes, selectors, empty_attr, Attribute, AttributeValue, Value},
        nodes::Node
    },
    duid_events::NodeMapMsg
};

/// TabNav View
pub fn tab_nav_view<M: Clone + 'static>(
    tab_nav_model: &TabNavModel,
    content: Node<TabNavMsg<M>>,
    tab_nav_item: Vec<(usize, Node<TabNavItemMsg>)>, 
    extra_tab_nav_item: Option<Vec<(usize, Node<ExtraTabNavItemMsg>)>>
) -> Node<TabNavMsg<M>> {
    
    /*
    let mut new_classes = vec!["tabnav-tabs".to_owned()];
    new_classes.extend_from_slice(&tab_nav_model.classes);
    */
    let classes_vec: Vec<_> = tab_nav_model.classes.iter().collect();
    let selectors_vec: Vec<_> = tab_nav_model.selectors.iter().collect();
    let header_classes_vec: Vec<_> = tab_nav_model.header_classes.iter().collect();
    let header_selectors_vec: Vec<_> = tab_nav_model.header_selectors.iter().collect();

    let nav_classes_vec: Vec<_> = tab_nav_model.nav_classes.iter().collect();
    let nav_selectors_vec: Vec<_> = tab_nav_model.nav_selectors.iter().collect();
    let extra_nav_classes_vec: Vec<_> = tab_nav_model.extra_nav_classes.iter().collect();
    let extra_nav_selectors_vec: Vec<_> = tab_nav_model.extra_nav_selectors.iter().collect();
    
    
    match extra_tab_nav_item {
        Some(extra_child) => {
            let tab_nav_item_view: Vec<_> = 
                tab_nav_item.iter()
                .map(|(index, child)| {
                    let i = index.clone();
                    child.to_owned().map_msg(move |tabnav_msg| TabNavMsg::Item((i, tabnav_msg)))
                })
                .collect();
            
            let extra_tab_nav_item_view: Vec<_> = 
                extra_child.iter()
                .map(|(index, child)| {
                    let i = index.clone();
                    child.to_owned().map_msg(move |extra_tabnav_msg| TabNavMsg::ExtraItem((i, extra_tabnav_msg)))
                })
                .collect();
            
            div(
                    &[
                        //classes(&["tabnav".to_owned()])
                        classes(&classes_vec),
                        selectors(&selectors_vec)
                    ],
                    &[
                        div(
                            &[
                                classes(&header_classes_vec),
                                selectors(&header_selectors_vec)
                            ],
                            &[
                                nav(
                                    &[
                                        classes(&nav_classes_vec),
                                        selectors(&nav_selectors_vec),
                                        Attribute::new(None, "aria-label", AttributeValue::from_value(Value::String(tab_nav_model.aria_label.clone()))),
                                    ],
                                    &tab_nav_item_view
                                ),
                                div(
                                    &[
                                        //classes(&["float-right".to_owned()])
                                        classes(&extra_nav_classes_vec),
                                        selectors(&extra_nav_selectors_vec)
                                    ],
                                    &extra_tab_nav_item_view
                                )
                            ]
                        ),
                        content
                    ]
            )
            
        },
        None => {
            let tab_nav_item_view: Vec<_> = 
                tab_nav_item.iter()
                .map(|(index, child)| {
                    let i = index.clone();
                    child.to_owned().map_msg(move |btn_grp_msg| TabNavMsg::Item((i, btn_grp_msg)))
                })
                .collect();

                div(
                    &[
                        //classes(&["tabnav".to_owned()])
                        classes(&classes_vec),
                        selectors(&selectors_vec)
                    ],
                    &[
                        div(
                            &[
                                classes(&header_classes_vec),
                                selectors(&header_selectors_vec)
                            ],
                            &[
                                nav(
                                    &[
                                        classes(&nav_classes_vec),
                                        selectors(&nav_selectors_vec),
                                        Attribute::new(None, "aria-label", AttributeValue::from_value(Value::String(tab_nav_model.aria_label.clone())))
                                    ],
                                    &tab_nav_item_view
                                )
                            ]
                        ),
                        content
                    ]
                )
        }
    }
}
