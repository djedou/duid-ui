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
        }
    }
};

// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum Messages {
    HeaderMsg,
    BodyMsg,
    FooterMsg
}


// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    HomePageMsg(PageMsg<Messages>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    page: PageModel
}


impl AppModel {
    pub fn new() -> Self {
        let mut page = PageModel::new();
        
        let mut classes = Vec::with_capacity(0);
        let mut selectors = Vec::with_capacity(0);
        selectors.push(".duid-page:::border border-solid border-neutral-400".to_owned());
        selectors.push(".duid-page > div:::border border-solid border-blue-400".to_owned());
        classes.push("duid-page".to_owned());

        page.add_selectors(&selectors);
        page.add_classes(&classes);

        AppModel {
            page
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {

    page_view::<Messages>(
        &app_model.page,
        Some(text_view(&TextModel::new(), "Page Header").map_msg(|_| Messages::HeaderMsg)),
        text_view(&TextModel::new(), "Page Body").map_msg(|_| Messages::BodyMsg),
        Some(text_view(&TextModel::new(), "Page Footer").map_msg(|_| Messages::FooterMsg)),
    ).map_msg(move |msg| AppMsg::HomePageMsg(msg))
}


pub fn app_update(_model: &mut AppModel, _msg: AppMsg) -> Cmd<AppMsg> {
    Cmd::none()
}

pub fn app_subscription(_model: &AppModel) -> Sub<AppMsg> {
    Sub::none()
}