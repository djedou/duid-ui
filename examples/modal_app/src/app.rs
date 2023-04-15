use duid_ui::{
    duid::{
        html::{
            div, button, summary,
            nodes::Node,
            attributes::{classes, r#type},
        },
        duid_events::{NodeMapMsg, Cmd, Sub}
    },
    components::{
        typography::{
            text::{text_view, TextModel},
        },
        layouts::modal::{
            ModalModel, ModalMsg, modal_view
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
    Modal(ModalMsg<Messages>)
}

impl From<ModalMsg<Messages>> for AppMsg {
    fn from(value: ModalMsg<Messages>) -> AppMsg {
        AppMsg::Modal(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    modal: ModalModel
}


impl AppModel {
    pub fn new() -> Self {
        let mut modal = ModalModel::new();
        modal.add_selectors(&[
            ".modal-container:::fixed z-6"
        ]);

        AppModel {
            modal
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {

    modal_view(
        summary(
            &[
                r#type("button".to_owned()),
            ],
            &[
                text_view(&TextModel::new(), "Modal").map_msg(|_| Messages::NoAction),
            ]
        ),
        &app_model.modal,
        modal_children(),
    ).map_msg(|m| AppMsg::Modal(m))
}


pub fn app_update(_model: &mut AppModel, msg: AppMsg) -> Cmd<AppMsg> {
    match msg {
        AppMsg::Modal(select) => {
            match select {
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
            classes(&["modal-container".to_owned()]),
        ],
        &[
            button(
                &[
                    classes(&["Modal-item".to_owned()]),
                ],
                &[
                    text_view(&TextModel::new(), "Item 1").map_msg(|_| Messages::NoAction),
                ]
            ),
            button(
                &[
                    classes(&["Modal-item".to_owned()]),
                ],
                &[
                    text_view(&TextModel::new(), "Item 2").map_msg(|_| Messages::NoAction),
                ]
            ),
            button(
                &[
                    classes(&["Modal-item".to_owned()]),
                ],
                &[
                    text_view(&TextModel::new(), "Item 3").map_msg(|_| Messages::NoAction),
                ]
            ),
        ]
    )
}