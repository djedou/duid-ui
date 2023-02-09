use duid_ui::{
    duid::{
        html::{
            text,
            nodes::Node,
        },
        duid_events::{NodeMapMsg, Cmd, Sub}
    },
    system::typography::TypographyModel,
    components::layouts::{
        holy_grail::{holy_grail_view, HolyGrailModel, HolyGrailMsg},
        sidebar::{sidebar_view, SidebarModel, SidebarMsg}
    }
};

// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum MessagesSide {
    Msg
}


// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum Messages {
    Msg(SidebarMsg<MessagesSide>)
}


// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    HolyGrail(HolyGrailMsg<Messages>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    typography: TypographyModel,
    holy_grail: HolyGrailModel,
    sidebar_model: SidebarModel
}


impl AppModel {
    pub fn new() -> Self {
        let typography = TypographyModel::new();
        let holy_grail = HolyGrailModel::new();
        let sidebar_model = SidebarModel::new();

        AppModel {
            typography,
            holy_grail,
            sidebar_model
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {



    holy_grail_view(
        &app_model.holy_grail,
        text("header"),
        Some(text("leftSide")),
        sidebar_view(
            &app_model.sidebar_model,
            text("main_sidebar"),
            text("main")
        ).map_msg(move |msg| Messages::Msg(msg)),
        None, //Some(text("rightSide")),
        Some(text("footer"))
    ).map_msg(move |msg| AppMsg::HolyGrail(msg))
}


pub fn app_update(_model: &mut AppModel, _msg: AppMsg) -> Cmd<AppMsg> {
    Cmd::none()
}

pub fn app_subscription(_model: &AppModel) -> Sub<AppMsg> {
    Sub::none()
}