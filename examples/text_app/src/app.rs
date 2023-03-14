use duid_ui::{
    duid::{
        html::{
            nodes::Node,
            attributes::{Attribute, AttributeValue, Value},
        },
        duid_events::{NodeMapMsg, Cmd, Sub}
    },
    components::typography::{
        text::{text_view, TextModel},
    }
};

// Messages
#[derive(Debug, PartialEq, Clone)]
pub struct AppMsg;

#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    text_model: TextModel
}

impl AppModel {
    pub fn new() -> Self {
        let mut text_model = TextModel::new();
        text_model.set_tag_span();
        text_model.add_classes(&["djed"]);
        text_model.add_selectors(&[".djed:::color--90-100-75", ".djed:hover:::color--300-100-75"]);
        text_model.add_attributes(&[
            Attribute::new(None, "title", AttributeValue::from_value(Value::String("Bravo Djedou".to_owned()))),
        ]);
        AppModel {
            text_model,
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {

    text_view(
        &app_model.text_model, 
        "Duid-UI is a powerfull UI librairy for anyone!"
    ).map_msg(|_| AppMsg)
}


pub fn app_update(_model: &mut AppModel, _msg: AppMsg) -> Cmd<AppMsg> {
    Cmd::none()
}

pub fn app_subscription(_model: &AppModel) -> Sub<AppMsg> {
    Sub::none()
}