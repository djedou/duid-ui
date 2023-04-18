use super::{InputModel, InputVariant, InputContext, InputValidation, get_normal_input_size_style, InputValidationNote,
    get_form_input_validation_style, get_normal_group_style, get_form_input_size_style, get_normal_group_note_style
};
use duid::{
        html::{
            div, input, p, label, span, 
            attributes::{disabled, classes, Attribute, AttributeValue, Value, selectors},
            nodes::{Node, create_fragment}
        },
        duid_events::NodeMapMsg,
        events::{InputEvent, on_input},
};


use std::rc::Rc;
use crate::{
    inputs::button::ButtonMsg,
    typography::{
        text::{text_view, TextModel},
    },
};
use super::InputButtonMsg;



/// Input View
pub fn input_view(input_model: &InputModel) -> Node<()>
{
    match &input_model.context {
        InputContext::Normal => {

            let mut new_label_classes = vec![];
            new_label_classes.extend_from_slice(&input_model.label_classes);
            
            let mut new_input_classes = vec![ "form-control".to_owned() ];
            get_normal_input_size_style(&input_model.size, &mut new_input_classes);
            new_input_classes.extend_from_slice(&input_model.input_classes);

            match &input_model.variant {
                InputVariant::Default => default_view(&input_model, &new_input_classes),
                InputVariant::Label => normal_group_view(&input_model, &new_input_classes, &new_label_classes, true),
                InputVariant::FloatLabel => normal_group_view(&input_model, &new_input_classes, &new_label_classes, false)
            }
        },
        InputContext::Form => {
            let mut new_label_classes = vec!["FormControl-label".to_owned()];
            new_label_classes.extend_from_slice(&input_model.label_classes);

            let mut new_input_classes = vec![ "FormControl-input".to_owned() ];
            get_form_input_size_style(&input_model.size, &mut new_input_classes);
            get_form_input_validation_style(&input_model.validation, &mut new_input_classes);
            new_input_classes.extend_from_slice(&input_model.input_classes);


            match &input_model.variant {
                InputVariant::Default => default_view(&input_model, &new_input_classes),
                InputVariant::Label => form_group_view(&input_model, &new_input_classes, &new_label_classes/*, true*/),
                InputVariant::FloatLabel => form_group_view(&input_model, &new_input_classes, &new_label_classes/*s, false*/)
            }
        }
    }
    
    
}


fn default_view(input_model: &InputModel, input_classes: &[String]) -> Node<()> {
    
    let new_input_value = Rc::clone(&input_model.value);
    let input_selectors: Vec<_> = input_model.selectors.iter().collect();
    let cb = Box::new({
        let new_input_value = Rc::clone(&new_input_value);

        move |e: InputEvent| {
            *new_input_value.borrow_mut() =  e.value;
        }
    }) as Box<dyn Fn(InputEvent)>;

    input(
        &[
            on_input(cb),
            classes(input_classes),
            selectors(&input_selectors),
            Attribute::new(None, "type", AttributeValue::from_value(Value::String(input_model.type_.to_string()))),
            Attribute::new(None, "placeholder", AttributeValue::from_value(Value::String(input_model.placeholder.clone()))),
            Attribute::new(None, "aria-label", AttributeValue::from_value(Value::String(input_model.aria_label.clone()))),
            Attribute::new(None, "value", AttributeValue::from_value(Value::String(input_model.value.borrow().clone()))),
            disabled(input_model.disabled),
        ],
        &[]
    )
}

fn normal_group_view(input_model: &InputModel, input_classes: &[String], label_classes: &[String], is_flat: bool) -> Node<()> {

    let new_classes = get_normal_group_style(&input_model.validation, is_flat);
    let input_selectors: Vec<_> = input_model.selectors.iter().collect();

    let new_input_value = Rc::clone(&input_model.value);
    let cb = Box::new({
        let new_input_value = Rc::clone(&new_input_value);

        move |e: InputEvent| {
            *new_input_value.borrow_mut() =  e.value;
        }
    }) as Box<dyn Fn(InputEvent)>;

    div(
        &[
            classes(&new_classes),
            selectors(&input_selectors),
        ],
        &[
            div(
                &[
                    classes(&["form-group-header".to_owned()]),
                ],
                &[
                    label(
                        &[
                            classes(label_classes),
                            Attribute::new(None, "for", AttributeValue::from_value(Value::String(input_model.input_id.clone()))),
                        ],
                        &[
                            text_view(&TextModel::new(), &input_model.label).map_msg(|_| ())
                        ]
                    ),
                ]
            ),
            div(
                &[
                    classes(&["form-group-body".to_owned()]),
                ],
                &[
                    input(
                        &[
                            on_input(cb),
                            classes(input_classes),
                            Attribute::new(None, "type", AttributeValue::from_value(Value::String(input_model.type_.to_string()))),
                            Attribute::new(None, "placeholder", AttributeValue::from_value(Value::String(input_model.placeholder.clone()))),
                            Attribute::new(None, "aria-label", AttributeValue::from_value(Value::String(input_model.aria_label.clone()))),
                            Attribute::new(None, "id", AttributeValue::from_value(Value::String(input_model.input_id.clone()))),
                            Attribute::new(None, "value", AttributeValue::from_value(Value::String(input_model.value.borrow().clone()))),
                            Attribute::new(None, "aria-describedby", AttributeValue::from_value(Value::String(format!("{}-validation", input_model.input_id.clone())))),
                            disabled(input_model.disabled)
                        ],
                        &[]
                    )
                ]
            ),
            normal_note_view(&input_model.validation, &input_model.validation_note, &input_model.input_id)
        ]
    )
}

fn form_group_view(input_model: &InputModel, input_classes: &[String], label_classes: &[String]/*, is_flat: bool*/) -> Node<()> {

    let new_input_value = Rc::clone(&input_model.value);
    let input_selectors: Vec<_> = input_model.selectors.iter().collect();
    let cb = Box::new({
        let new_input_value = Rc::clone(&new_input_value);

        move |e: InputEvent| {
            *new_input_value.borrow_mut() =  e.value;
        }
    }) as Box<dyn Fn(InputEvent)>;

    div(
        &[
            if input_model.full_width {
                classes(&["FormControl".to_owned(), "FormControl--fullWidth".to_owned()])
            }  
            else {
                classes(&["FormControl".to_owned()])
            },
            selectors(&input_selectors),
        ],
        &[
            label(
                &[
                    classes(label_classes),
                    Attribute::new(None, "for", AttributeValue::from_value(Value::String(input_model.input_id.clone()))),
                ],
                &[ 
                    text_view(&TextModel::new(), &input_model.label).map_msg(|_| ())
                ]
            ),
            input(
                &[
                    on_input(cb),
                    classes(input_classes),
                    Attribute::new(None, "type", AttributeValue::from_value(Value::String(input_model.type_.to_string()))),
                    Attribute::new(None, "placeholder", AttributeValue::from_value(Value::String(input_model.placeholder.clone()))),
                    Attribute::new(None, "aria-label", AttributeValue::from_value(Value::String(input_model.aria_label.clone()))),
                    Attribute::new(None, "id", AttributeValue::from_value(Value::String(input_model.input_id.clone()))),
                    Attribute::new(None, "value", AttributeValue::from_value(Value::String(input_model.value.borrow().clone()))),
                    disabled(input_model.disabled)
                ],
                &[]
            )
        ]
    )
}

pub fn input_add_on_appended_view(input_model: &InputModel, button: Node<ButtonMsg>) -> Node<InputButtonMsg> {
    
    let mut new_input_classes = vec![ "form-control".to_owned()];
    let input_selectors: Vec<_> = input_model.selectors.iter().collect();

    get_normal_input_size_style(&input_model.size, &mut new_input_classes);
    new_input_classes.extend_from_slice(&input_model.input_classes);

    let new_input_value = Rc::clone(&input_model.value);
    let cb = Box::new({
        let new_input_value = Rc::clone(&new_input_value);

        move |e: InputEvent| {
            *new_input_value.borrow_mut() =  e.value;
        }
    }) as Box<dyn Fn(InputEvent)>;

    div(
        &[
            if input_model.input_group_inline {
                classes(&["input-group".to_owned(), "inline".to_owned()])
            }
            else {
                classes(&["input-group".to_owned()])
            },
            selectors(&input_selectors),
        ],
        &[
            input(
                &[
                    on_input(cb),
                    classes(&new_input_classes),
                    Attribute::new(None, "type", AttributeValue::from_value(Value::String(input_model.type_.to_string()))),
                    Attribute::new(None, "placeholder", AttributeValue::from_value(Value::String(input_model.placeholder.clone()))),
                    Attribute::new(None, "aria-label", AttributeValue::from_value(Value::String(input_model.aria_label.clone()))),
                    Attribute::new(None, "value", AttributeValue::from_value(Value::String(input_model.value.borrow().clone()))),
                    disabled(input_model.disabled)
                ],
                &[]
            ).map_msg(move |input_msg| InputButtonMsg::InputMsg(input_msg)),
            span(
                &[
                    classes(&["input-group-button".to_owned()])
                ],
                &[
                    button.map_msg(move |btn_msg| InputButtonMsg::ButtonMsg(btn_msg))
                ]
            )
        ]
    )
}

fn normal_note_view(validation: &InputValidation, note: &InputValidationNote, id: &str) -> Node<()> {
    match validation {
        InputValidation::Successed => {
            p(
                &[
                    classes(&get_normal_group_note_style(&validation)),
                    Attribute::new(None, "id", AttributeValue::from_value(Value::String(format!("{}-validation", id)))),
                ],
                &[
                    text_view(&TextModel::new(), &note.successed).map_msg(|_| ())
                ]
            )
        },
        InputValidation::Errored => {
            p(
                &[
                    classes(&get_normal_group_note_style(&validation)),
                    Attribute::new(None, "id", AttributeValue::from_value(Value::String(format!("{}-validation", id)))),
                ],
                &[
                    text_view(&TextModel::new(), &note.errored).map_msg(|_| ())
                ]
            )
        },
        InputValidation::Warn => {
            p(
                &[
                    classes(&get_normal_group_note_style(&validation)),
                    Attribute::new(None, "id", AttributeValue::from_value(Value::String(format!("{}-validation", id)))),
                ],
                &[
                    text_view(&TextModel::new(), &note.warn).map_msg(|_| ())
                ]
            )
        },
        InputValidation::None => {
            create_fragment(&[])
        }
    }
}