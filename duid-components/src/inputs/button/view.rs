use super::{ButtonMsg, ButtonModel, ButtonKind, ButtonIcon, ButtonState, ButtonColor, ButtonSize, ButtonVariation};
use duid::{
        html::{
            button, span, a, summary,
            attributes::{classes, selectors, Attribute, AttributeValue, Value, disabled, empty_attr},
            nodes::Node
        },
        events,
        duid_events::{NodeMapMsg}    
};
use crate::typography::text::{TextModel, text_view};
use std::collections::HashSet;



pub fn button_view(button_model: &ButtonModel, child: Node<ButtonMsg>, icon: Option<Node<ButtonMsg>>) -> Node<ButtonMsg> {
    
    match &button_model.variation {
        &ButtonVariation::Link => {
            a(
                &build_attributes(&button_model),
                &[child]
            )
        },
        &ButtonVariation::Summary => {
            match button_model.icon {
                ButtonIcon::None => {
                    summary(
                        &build_attributes(&button_model),
                        &[child]
                    )
                },
                ButtonIcon::HiddenText => {
                    summary(
                        &[
                            classes(&["hidden-text-expander inline".to_owned()])
                        ],
                        &[
                            button(
                                &build_attributes(&button_model),
                                &[
                                    text_view(&TextModel::new(), "...")
                                        .map_msg(|_| ButtonMsg::NoAction)
                                ]
                            )
                        ]
                    )
                },
                ButtonIcon::With => {
                    summary(
                        &build_attributes(&button_model),
                        &[
                            icon.unwrap_or(span(&[],&[])),
                            child
                        ]
                    )
                },
                ButtonIcon::Only | ButtonIcon::Close => {
                    summary(
                        &build_attributes(&button_model),
                        &[
                            icon.unwrap_or(span(&[],&[])),
                        ]
                    )
                },
                ButtonIcon::Count => {
                    summary(
                        &build_attributes(&button_model),
                        &[
                            child,
                            text_view(&TextModel::new(), &button_model.count.to_string()).map_msg(|_| ButtonMsg::NoAction)
                        ]
                    )
                }
            }
        },
        _ => {
            match button_model.icon {
                ButtonIcon::None => {
                    button(
                        &build_attributes(&button_model),
                        &[child]
                    )
                },
                ButtonIcon::HiddenText => {
                    span(
                        &[
                            classes(&["hidden-text-expander inline".to_owned()])
                        ],
                        &[
                            button(
                                &build_attributes(&button_model),
                                &[
                                    text_view(&TextModel::new(), "...")
                                        .map_msg(|_| ButtonMsg::NoAction)
                                ]
                            )
                        ]
                    )
                },
                ButtonIcon::With => {
                    button(
                        &build_attributes(&button_model),
                        &[
                            icon.unwrap_or(span(&[],&[])),
                            child
                        ]
                    )
                },
                ButtonIcon::Only | ButtonIcon::Close => {
                    button(
                        &build_attributes(&button_model),
                        &[
                            icon.unwrap_or(span(&[],&[])),
                        ]
                    )
                },
                ButtonIcon::Count => {
                    button(
                        &build_attributes(&button_model),
                        &[
                            child,
                            text_view(&TextModel::new(), &button_model.count.to_string()).map_msg(|_| ButtonMsg::NoAction)
                        ]
                    )
                }
            }
        }
    }
}

fn build_attributes(button_model: &ButtonModel) -> Vec<Attribute<ButtonMsg>> {

    let mut button_classes = HashSet::with_capacity(0);
    button_classes.extend(&button_model.classes);

    let mut button_selectors = HashSet::with_capacity(0);
    button_selectors.extend(&button_model.selectors);
    

    let button_classes_vec: Vec<_> = button_classes.iter().collect();
    let button_selectors_vec: Vec<_> = button_selectors.iter().collect();

    let mut button_attributes = 
        vec![
            classes(&button_classes_vec),
            selectors(&button_selectors_vec),
            Attribute::new(None, "type", AttributeValue::from_value(Value::String("button".to_owned()))),
            if button_model.is_group_item {
                classes(&["BtnGroup-item".to_owned()])
            }
            else {
                empty_attr()
            }
        ];
    
        match &button_model.variation {
            &ButtonVariation::Normal | &ButtonVariation::Block | &ButtonVariation::Summary => {
                // Set Button Kind attributes
                match &button_model.kind {
                    &ButtonKind::Default => {
                        if let Some(kind_class) = button_model.kind_classes.get(&ButtonKind::Default) {
                            let kind_class_vec: Vec<_> = kind_class.iter().collect();
                            button_attributes.extend([classes(&kind_class_vec)]);
                        }
                    },
                    &ButtonKind::Filled => {
                        if let Some(kind_class) = button_model.kind_classes.get(&ButtonKind::Filled) {
                            let kind_class_vec: Vec<_> = kind_class.iter().collect();
                            button_attributes.extend([classes(&kind_class_vec)]);
                        }
                    },
                    &ButtonKind::Outline => {
                        if let Some(kind_class) = button_model.kind_classes.get(&ButtonKind::Outline) {
                            let kind_class_vec: Vec<_> = kind_class.iter().collect();
                            button_attributes.extend([classes(&kind_class_vec)]);
                        }
                    }
                };
        
                // Set Button Colors attributes
                match &button_model.colors {
                    &ButtonColor::Default => {},
                    &ButtonColor::FilledDefault => {
                        if let Some(colors_class) = button_model.colors_classes.get(&ButtonColor::FilledDefault) {
                            let colors_class_vec: Vec<_> = colors_class.iter().collect();
                            button_attributes.extend([classes(&colors_class_vec)]);
                        }
                    },
                    &ButtonColor::FilledDanger => {
                        if let Some(colors_class) = button_model.colors_classes.get(&ButtonColor::FilledDanger) {
                            let colors_class_vec: Vec<_> = colors_class.iter().collect();
                            button_attributes.extend([classes(&colors_class_vec)]);
                        }
                    },
                    &ButtonColor::OutlineDefault => {
                        if let Some(colors_class) = button_model.colors_classes.get(&ButtonColor::OutlineDefault) {
                            let colors_class_vec: Vec<_> = colors_class.iter().collect();
                            button_attributes.extend([classes(&colors_class_vec)]);
                        }
                    },
                    &ButtonColor::OutlineDanger => {
                        if let Some(colors_class) = button_model.colors_classes.get(&ButtonColor::OutlineDanger) {
                            let colors_class_vec: Vec<_> = colors_class.iter().collect();
                            button_attributes.extend([classes(&colors_class_vec)]);
                        }
                    }
                };
        
                // Set Button Size attributes
                match &button_model.size {
                    &ButtonSize::Small => {
                        if let Some(size_class) = button_model.size_classes.get(&ButtonSize::Small) {
                            let size_class_vec: Vec<_> = size_class.iter().collect();
                            button_attributes.extend([classes(&size_class_vec)]);
                        }
                    },
                    &ButtonSize::Medium => {
                        if let Some(size_class) = button_model.size_classes.get(&ButtonSize::Medium) {
                            let size_class_vec: Vec<_> = size_class.iter().collect();
                            button_attributes.extend([classes(&size_class_vec)]);
                        }
                    },
                    &ButtonSize::Large => {
                        if let Some(size_class) = button_model.size_classes.get(&ButtonSize::Large) {
                            let size_class_vec: Vec<_> = size_class.iter().collect();
                            button_attributes.extend([classes(&size_class_vec)]);
                        }
                    }
                };
        
                // Set Button Icon attributes
                match &button_model.icon {
                    &ButtonIcon::Only => {
                        if let Some(icon_class) = button_model.icon_classes.get(&ButtonIcon::Only) {
                            let icon_class_vec: Vec<_> = icon_class.iter().collect();
                            button_attributes.extend([classes(&icon_class_vec)]);
                        }
                    },
                    &ButtonIcon::Close => {
                        if let Some(icon_class) = button_model.icon_classes.get(&ButtonIcon::Close) {
                            let icon_class_vec: Vec<_> = icon_class.iter().collect();
                            button_attributes.extend([classes(&icon_class_vec)]);
                        }
                    },
                    &ButtonIcon::Count => {
                        if let Some(icon_class) = button_model.icon_classes.get(&ButtonIcon::Count) {
                            let icon_class_vec: Vec<_> = icon_class.iter().collect();
                            button_attributes.extend([classes(&icon_class_vec)]);
                        }
                    },
                    &ButtonIcon::HiddenText => {
                        if let Some(icon_class) = button_model.icon_classes.get(&ButtonIcon::HiddenText) {
                            let icon_class_vec: Vec<_> = icon_class.iter().collect();
                            button_attributes.extend([
                                classes(&icon_class_vec),
                                Attribute::new(None, "aria-expanded", AttributeValue::from_value(Value::Bool(false))),
                            ]);
                        }
                    },
                    _ => {}
                };
        
                match button_model.state {
                    ButtonState::Disabled => {
                        button_attributes.extend_from_slice(&[
                            Attribute::new(None, "aria-disabled", AttributeValue::from_value(Value::Bool(true))),
                            disabled(true),
                            Attribute::new(None, "aria-selected", AttributeValue::from_value(Value::Bool(false)))
                        ]);
                    },
                    ButtonState::Selected => {
                        button_attributes.extend_from_slice(&[
                            Attribute::new(None, "aria-disabled", AttributeValue::from_value(Value::Bool(false))),
                            disabled(false),
                            Attribute::new(None, "aria-selected", AttributeValue::from_value(Value::Bool(true)))
                        ]);
                    },
                    ButtonState::Normal => {
                        button_attributes.extend_from_slice(&[
                            Attribute::new(None, "aria-disabled", AttributeValue::from_value(Value::Bool(false))),
                            disabled(false),
                            Attribute::new(None, "aria-selected", AttributeValue::from_value(Value::Bool(false)))
                        ]);
                    }
                }
            },
            _ => {},
        };

    // Set Button Variation
    match &button_model.variation {
        &ButtonVariation::Normal => {
            if let Some(normal_class) = button_model.variation_classes.get(&ButtonVariation::Normal) {
                let normal_class_vec: Vec<_> = normal_class.iter().collect();
                button_attributes.extend([
                    classes(&normal_class_vec),
                    events::on_click(|_| ButtonMsg::OnClick)
                ]);
            }
        },
        &ButtonVariation::Block => {
            if let Some(block_class) = button_model.variation_classes.get(&ButtonVariation::Block) {
                let block_class_vec: Vec<_> = block_class.iter().collect();
                button_attributes.extend([
                    classes(&block_class_vec),
                    events::on_click(|_| ButtonMsg::OnClick),
                ]);
            }
        },
        &ButtonVariation::Link => {
            if let Some(link_class) = button_model.variation_classes.get(&ButtonVariation::Link) {
                let link_class_vec: Vec<_> = link_class.iter().collect();
                button_attributes.extend([
                    classes(&link_class_vec),
                    events::on_click(|_| ButtonMsg::OnClick)
                ]);
            }
        },
        &ButtonVariation::Summary => {
            if let Some(summary_class) = button_model.variation_classes.get(&ButtonVariation::Summary) {
                let summary_class_vec: Vec<_> = summary_class.iter().collect();
                button_attributes.extend([
                    classes(&summary_class_vec),
                    Attribute::new(None, "aria-haspopup", AttributeValue::from_value(Value::Bool(true))),
                    events::on_click(|_| ButtonMsg::OnClick)
                ]);
            }
        }
    };

    button_attributes.extend_from_slice(&button_model.attributes);
    button_attributes
}
