use duid_ui::{
    duid::{
        html::{
            div, button,
            nodes::Node,
            attributes::classes
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
        text_view(&TextModel::new(), "menu").map_msg(|_| SelectMenuMsg::NoAction),
        None,
        modal_children(),
        None
    ).map_msg(|m| AppMsg::SelectMenu(m))
}


pub fn app_update(_model: &mut AppModel, _msg: AppMsg) -> Cmd<AppMsg> {
    Cmd::none()
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
                ],
                &[
                    text_view(&TextModel::new(), "Item 1").map_msg(|_| Messages::NoAction),
                ]
            ),
            button(
                &[
                    classes(&["SelectMenu-item".to_owned()]),
                ],
                &[
                    text_view(&TextModel::new(), "Item 2").map_msg(|_| Messages::NoAction),
                ]
            ),
            button(
                &[
                    classes(&["SelectMenu-item".to_owned()]),
                ],
                &[
                    text_view(&TextModel::new(), "Item 3").map_msg(|_| Messages::NoAction),
                ]
            ),
        ]
    )
}