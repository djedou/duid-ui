use duid_ui::{
    duid::{
        html::{
            text,
            nodes::Node,
        },
        duid_events::{NodeMapMsg, Cmd, Sub}
    },
    components::{
        layouts::{
            page::{page_view, PageModel, PageMsg}
        },
        typography::{
            text::{text_view, TextModel},
        },
        navigation::{
            tab_nav::{TabNavMsg}
        },
    }
};
use crate::tab::{TabAppMsg, TabAppModel, tab_app_view, tab_app_update, tab_app_subscription};


// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    TabMsg(TabAppMsg)
}

impl From<TabAppMsg> for AppMsg {
    fn from(value: TabAppMsg) -> AppMsg {
        AppMsg::TabMsg(value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    tab_app: TabAppModel
}


impl AppModel {
    pub fn new() -> Self {

        AppModel {
            tab_app: TabAppModel::new()
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {

    tab_app_view(
        &app_model.tab_app
    ).map_msg(|m| AppMsg::TabMsg(m))
}


pub fn app_update(model: &mut AppModel, msg: AppMsg) -> Cmd<AppMsg> {
    match msg {
        AppMsg::TabMsg(tab_msg) => {
            tab_app_update(&mut model.tab_app, tab_msg).map_cmd_msg::<AppMsg>()
        }
    }
}

pub fn app_subscription(_model: &AppModel) -> Sub<AppMsg> {
    Sub::none()
}