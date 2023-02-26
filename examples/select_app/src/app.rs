use duid_ui::{
    duid::{
        html::{
            div, button,
            nodes::Node,
            attributes::{classes, Attribute, AttributeValue, Value},
        },
        duid_events::{NodeMapMsg, Cmd, Sub}
    },
    components::{
        typography::{
            text::{text_view, TextModel},
        },
        inputs::selects::{
            SelectMenuModel, SelectMenuMsg, select_menu_view
        },
    }
};

// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum Messages {
    NoAction
}

// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    SelectMenu(SelectMenuMsg<Messages>)
}

impl From<SelectMenuMsg<Messages>> for AppMsg {
    fn from(value: SelectMenuMsg<Messages>) -> AppMsg {
        AppMsg::SelectMenu(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    select_menu: SelectMenuModel
}


impl AppModel {
    pub fn new() -> Self {
        AppModel {
            select_menu: SelectMenuModel::new()
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {

    select_menu_view(
        &app_model.select_menu,
        None,
        modal_children(),
        None
    ).map_msg(|m| AppMsg::SelectMenu(m))
}


pub fn app_update(_model: &mut AppModel, msg: AppMsg) -> Cmd<AppMsg> {
    match msg {
        AppMsg::SelectMenu(select) => {
            match select {
                SelectMenuMsg::Button(_) => {
                    Cmd::none()
                },
                _ => Cmd::none()
            }
        }
    }
}

pub fn app_subscription(_model: &AppModel) -> Sub<AppMsg> {
    Sub::none()
}

fn modal_children() -> Node<Messages> {
    div(
        &[
            classes(&["SelectMenu-list".to_owned()])
        ],
        &[
            button(
                &[
                    classes(&["SelectMenu-item".to_owned()]),
                    Attribute::new(None, "role", AttributeValue::from_value(Value::String("menuitem".to_string())))
                ],
                &[
                    text_view(&TextModel::new(), "Item 1").map_msg(|_| Messages::NoAction),
                ]
            ),
            button(
                &[
                    classes(&["SelectMenu-item".to_owned()]),
                    Attribute::new(None, "role", AttributeValue::from_value(Value::String("menuitem".to_string())))
                ],
                &[
                    text_view(&TextModel::new(), "Item 2").map_msg(|_| Messages::NoAction),
                ]
            ),
            button(
                &[
                    classes(&["SelectMenu-item".to_owned()]),
                    Attribute::new(None, "role", AttributeValue::from_value(Value::String("menuitem".to_string())))
                ],
                &[
                    text_view(&TextModel::new(), "Item 3").map_msg(|_| Messages::NoAction),
                ]
            ),
        ]
    )
}