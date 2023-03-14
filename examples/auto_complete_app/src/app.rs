use duid_ui::{
    duid::{
        html::{
            div, button,
            nodes::Node,
            attributes::classes,
        },
        duid_events::{NodeMapMsg, Cmd, Sub}
    },
    components::{
        typography::{
            text::{text_view, TextModel},
        },
        inputs::forms::{
            AutoCompleteModel, AutoCompleteMsg, auto_complete_view
        },
    }
};


// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    AutoCompleteMsg(AutoCompleteMsg)
}
/*
impl From<SelectMenuMsg<Messages>> for AppMsg {
    fn from(value: SelectMenuMsg<Messages>) -> AppMsg {
        AppMsg::SelectMenu(value)
    }
}
*/
#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    auto_complete: AutoCompleteModel
}


impl AppModel {
    pub fn new() -> Self {
        AppModel {
            auto_complete: AutoCompleteModel::new()
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {

    auto_complete_view(
        &app_model.auto_complete,
    ).map_msg(|m| AppMsg::AutoCompleteMsg(m))
}


pub fn app_update(_model: &mut AppModel, msg: AppMsg) -> Cmd<AppMsg> {
    Cmd::none()
}

pub fn app_subscription(_model: &AppModel) -> Sub<AppMsg> {
    Sub::none()
}


/*
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
*/