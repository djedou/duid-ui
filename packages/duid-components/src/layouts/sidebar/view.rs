use super::{SidebarModel, SidebarMsg};
use duid::{
    html::{
        div, aside, main, 
        attributes::{style, classes, empty_attr},
        nodes::Node
    },
    duid_events::NodeMapMsg,
};
use duid_system::{
    flexbox::Flexbox,
    heights::Height,
    display::Display,
    z_index::ZIndex,
    media_query::BreakPoint
};


/// Sidebar View
pub fn sidebar_view<M>(
    sidebar_model: &SidebarModel,
    side_view: Node<M>,
    main_view: Node<M>
) -> Node<SidebarMsg<M>>
where 
    M: Clone + 'static
{
    let mut new_classes = vec![
        Flexbox::Flex(None).to_string(),
        Height::H100(None).to_string()
    ];
    new_classes.extend_from_slice(&sidebar_model.root_classes);

    //#### Main ####
    let mut new_main_classes = vec![
        Flexbox::Grow1(None).to_string()
    ];
    new_main_classes.extend_from_slice(&sidebar_model.main_classes);


    //#### SideBar ####
    let mut new_sidebar_classes = vec![
        ZIndex::Z100.to_string(),
        Display::Block(Some(BreakPoint::NotSmall)).to_string()
    ];
    if sidebar_model.hide_mobile_sidebar {
        new_sidebar_classes.push(
            Display::None(None).to_string()
        )  
    }
    new_sidebar_classes.extend_from_slice(&sidebar_model.sidebar_classes);


    div(
        &[
            classes(&new_classes),
            if let Some((name, s_value)) = &sidebar_model.root_style {
                style(name, s_value)
            }
            else {
                empty_attr()
            },
        ],
        &[
            main(
                &[
                    
                    classes(&new_main_classes),
                    if let Some((name, s_value)) = &sidebar_model.main_style {
                        style(name, s_value)
                    }
                    else {
                        empty_attr()
                    },
                ],
                &[main_view.map_msg(move |msg| SidebarMsg::Element(msg))]
            ),
            aside(
                &[
                    classes(&new_sidebar_classes),
                    if let Some((name, s_value)) = &sidebar_model.sidebar_style {
                        style(name, s_value)
                    }
                    else {
                        empty_attr()
                    },
                ],
                &[side_view.map_msg(move |msg| SidebarMsg::Element(msg))]
            )
        ]
    )   
}