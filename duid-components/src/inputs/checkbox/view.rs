use super::{InputContext, CheckboxModel
};
use shared::{
    duid::{
        v_dom::html::{div, input, p, text, label, em, attributes::{style, disabled, classes, /*empty_attr,*/ Attribute, AttributeValue, Value}},
        v_dom::v_node::{Node, create_fragment},
        v_dom::events::{on_checked},
        //event_manager::map_msg::{NodeMapMsg}
    }
};
use std::rc::Rc;


/// Checkbox View
pub fn checkbox_view(checkbox_model: &CheckboxModel, help_message: Node<()>) -> Node<()>
{
    match &checkbox_model.context {
        InputContext::Normal => {
            normal_form_group(&checkbox_model, help_message)
        },
        InputContext::Form => {
            /*let mut new_label_classes = vec!["FormControl-label".to_owned()];
            new_label_classes.extend_from_slice(&input_model.label_classes);

            let mut new_input_classes = vec![ "FormControl-input".to_owned() ];
            get_form_input_size_style(&input_model.size, &mut new_input_classes);
            get_form_input_validation_style(&input_model.validation, &mut new_input_classes);
            new_input_classes.extend_from_slice(&input_model.input_classes);


            match &input_model.variant {
                InputVariant::Default => default_view(&input_model, &new_input_classes),
                InputVariant::Label => form_group_view(&input_model, &new_input_classes, &new_label_classes/*, true*/),
                InputVariant::FloatLabel => form_group_view(&input_model, &new_input_classes, &new_label_classes/*s, false*/)
            }*/

            create_fragment(&[])
        }
    }
    
    
}

fn normal_form_group(checkbox_model: &CheckboxModel, help_message: Node<()>) -> Node<()> {

    let mut new_label_classes = Vec::with_capacity(0);
    new_label_classes.extend_from_slice(&checkbox_model.label_classes);
    
    let mut new_input_classes = Vec::with_capacity(0);
    new_input_classes.extend_from_slice(&checkbox_model.input_classes);

    let new_input_checked = Rc::clone(&checkbox_model.checked);
    let cb = Box::new({
        let new_input_checked = Rc::clone(&new_input_checked);

        move |e: bool| {
            *new_input_checked.borrow_mut() =  e;
        }
    }) as Box<dyn Fn(bool)>;

    div(
        &[
            classes(&["form-checkbox".to_owned()])
        ],
        &[
            label(
                &[
                    classes(&new_label_classes),
                ],
                &[
                    input(
                        &[
                            on_checked(cb),
                            classes(&new_input_classes),
                            Attribute::new(None, "type", AttributeValue::from_value(Value::String(checkbox_model.type_.to_string()))),
                            //Attribute::new(None, "checked", AttributeValue::from_value(Value::String("checked".to_owned()))),
                            
                            disabled(checkbox_model.disabled),
                            Attribute::new(None, "aria-describedby", AttributeValue::from_value(Value::String(format!("{}-for-checkbox", checkbox_model.input_id.clone())))),
                            style("duid-forms-styles", String::with_capacity(0)),
                        ],
                        &[]
                    ),
                    if checkbox_model.emphasized {
                        em(
                            &[
                                classes(&["highlight".to_owned()])
                            ],
                            &[
                                text(&[], &checkbox_model.label)
                            ]
                        )
                    }
                    else {
                        text(&[], &checkbox_model.label) 
                    }
                ]
            ),
            if checkbox_model.show_note {
                p(
                    &[
                        classes(&["note".to_owned()]),
                        Attribute::new(None, "id", AttributeValue::from_value(Value::String(format!("{}-for-checkbox", checkbox_model.input_id.clone())))),
                    ],
                    &[
                        help_message
                    ]
                )
            }
            else {
                create_fragment(&[])
            }
        ]
    )
}
