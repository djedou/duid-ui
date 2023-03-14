use super::{TabNavModel, TabNavItemMsg, TabNavMsg, ExtraTabNavItemMsg};
use duid::{
    html::{
        nav, div,
        attributes::{classes, selectors, Attribute, AttributeValue, Value},
        nodes::Node
    },
    duid_events::NodeMapMsg
};

/// TabNav View
pub fn tab_nav_view(
    tab_nav_model: &TabNavModel,
    tab_nav_item: Vec<(usize, Node<TabNavItemMsg>)>, 
    extra_tab_nav_item: Option<Vec<(usize, Node<ExtraTabNavItemMsg>)>>
) -> Node<TabNavMsg> {
    
    let classes_vec: Vec<_> = tab_nav_model.classes.iter().collect();
    let selectors_vec: Vec<_> = tab_nav_model.selectors.iter().collect();
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
                    classes(&classes_vec),
                    selectors(&selectors_vec)
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
                            classes(&extra_nav_classes_vec),
                            selectors(&extra_nav_selectors_vec)
                        ],
                        &extra_tab_nav_item_view
                    )
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
                        classes(&classes_vec),
                        selectors(&selectors_vec)
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
                )
        }
    }
}
