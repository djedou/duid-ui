use super::{HolyGrailModel, HolyGrailMsg};
use duid::{
    html::{
        div, aside, nav, header, article, main, footer, 
        attributes::{style, classes, empty_attr},
        nodes::Node
    },
    duid_events::NodeMapMsg,
};
use duid_system::{
    flexbox::Flexbox,
    widths::Width,
    heights::Height,
    position::Position,
    coordinates::Coordinate,
    z_index::ZIndex,
    display::Display,
    media_query::BreakPoint
};

/// HolyGrail View
pub fn holy_grail_view<M>(
    holy_grail_model: &HolyGrailModel,
    header_view: Node<M>,
    left_side_view: Option<Node<M>>,
    main_view: Node<M>,
    right_side_view: Option<Node<M>>,
    footer_view: Option<Node<M>>
) -> Node<HolyGrailMsg<M>>
where 
    M: Clone + 'static
{
    
    let mut new_classes = vec![
        Flexbox::Flex(None).to_string(),
        Flexbox::FlexColumn(None).to_string(),
        Height::Vh100(None).to_string()
    ];
    new_classes.extend_from_slice(&holy_grail_model.holy_grail_root_classes);

    //##### Header ######
    let mut new_header_classes = vec![
        Flexbox::Shrink1(None).to_string(),
        Width::W100(None).to_string(),
        ZIndex::Z100.to_string(),
        holy_grail_model.header_height.clone().to_string()
    ];
    new_header_classes.extend_from_slice(&holy_grail_model.header_classes);

    //#### Main ######
    let mut main_view_children = vec![];

    //##### Left Side #####
    if let Some(l_side_view) = left_side_view {
        let mut new_left_side_classes = vec![
            ZIndex::Z100.to_string(),
            Display::Block(Some(BreakPoint::NotSmall)).to_string()
        ];
        if holy_grail_model.hide_mobile_left_sidebar {
            new_left_side_classes.push(
                Display::None(None).to_string()
            )  
        }
        new_left_side_classes.extend_from_slice(&holy_grail_model.left_sidebar_classes);

        main_view_children.push(
            aside(
                &[
                    classes(&new_left_side_classes),
                    if let Some((name, s_value)) = &holy_grail_model.left_sidebar_style {
                        style(name, s_value)
                    }
                    else {
                        empty_attr()
                    },
                ],
                &[l_side_view.map_msg(move |msg| HolyGrailMsg::Element(msg))]
            ),
        )
    }

    //### Middle ##### 
    let mut new_body_classes = vec![
        Flexbox::Grow1(None).to_string()
    ];
    new_body_classes.extend_from_slice(&holy_grail_model.body_classes);

    main_view_children.push(
        article(
            &[
                classes(&new_body_classes),
                if let Some((name, s_value)) = &holy_grail_model.body_style {
                    style(name, s_value)
                }
                else {
                    empty_attr()
                },
            ],
            &[
                main_view.map_msg(move |aside_msg| HolyGrailMsg::Element(aside_msg))
            ]
        )
    );

    //##### Right Side #####
    if let Some(side_view) = right_side_view {
        let mut new_right_side_classes = vec![
            ZIndex::Z100.to_string(),
            Display::Block(Some(BreakPoint::NotSmall)).to_string()
        ];
        if holy_grail_model.hide_mobile_right_sidebar {
            new_right_side_classes.push(
                Display::None(None).to_string()
            )  
        }
        new_right_side_classes.extend_from_slice(&holy_grail_model.right_sidebar_classes);

        main_view_children.push(
            nav(
                &[
                    classes(&new_right_side_classes),
                    if let Some((name, s_value)) = &holy_grail_model.right_sidebar_style {
                        style(name, s_value)
                    }
                    else {
                        empty_attr()
                    },
                ],
                &[side_view.map_msg(move |msg| HolyGrailMsg::Element(msg))]
            ),
        )
    }

    //#### Root children ####
    let mut root_children = vec![
        header(
            &[
                classes(&new_header_classes),
                if let Some((name, s_value)) = &holy_grail_model.header_style {
                    style(name, s_value)
                }
                else {
                    empty_attr()
                },
            ],
            &[
                header_view.map_msg(move |header_msg| HolyGrailMsg::Element(header_msg)),
            ]
        ),
        main(
            &[
                classes(&[
                    Flexbox::Grow1(None).to_string(),
                    Flexbox::Flex(None).to_string(),
                    Flexbox::FlexRow(None).to_string(),
                    Position::Absolute(None).to_string(),
                    Coordinate::Left0(None).to_string(),
                    Coordinate::Right0(None).to_string(),
                    holy_grail_model.header_height.into_coordinate_top(),
                    holy_grail_model.footer_height.into_coordinate_bottom()
                ])
            ],
            &main_view_children
        )
    ];

    //##### Footer ######
    if let Some(f_view) = footer_view {
        let mut new_footer_classes = vec![
            Position::Absolute(None).to_string(),
            Width::W100(None).to_string(),
            Coordinate::Left0(None).to_string(),
            Coordinate::Bottom0(None).to_string(),
            ZIndex::Z100.to_string(),
            holy_grail_model.footer_height.clone().to_string()
        ];

        new_footer_classes.extend_from_slice(&holy_grail_model.footer_classes);
        root_children.push(
            footer(
                &[
                    classes(&new_footer_classes),
                    if let Some((name, s_value)) = &holy_grail_model.holy_grail_root_style {
                        style(name, s_value)
                    }
                    else {
                        empty_attr()
                    },
                ],
                &[f_view.map_msg(move |msg| HolyGrailMsg::Element(msg))]
            )
        )
    }
    
    div(
        &[
            classes(&new_classes),
            if let Some((name, s_value)) = &holy_grail_model.holy_grail_root_style {
                style(name, s_value)
            }
            else {
                empty_attr()
            },
            style("duid-holy-grail-styles", String::with_capacity(0)),
        ],
        &root_children
    )
}
