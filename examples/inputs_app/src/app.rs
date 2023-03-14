/*
use duid_ui::components::{
    inputs::{
        forms::input::{InputModel, InputButtonMsg, input_view, input_add_on_appended_view},
        button::{button_view, ButtonModel, ButtonMsg},
        
    }
};
*/
use duid_ui::{
    duid::{
        html::{
            div, text,
            nodes::Node,
            attributes::{classes, selectors},
        },
        duid_events::{NodeMapMsg, Cmd, Sub}
    },
    components::{
        typography::text::{text_view, TextModel},
        inputs::{
            buttons::{ButtonModel, ButtonMsg, button_view},
            forms::{InputModel, InputButtonMsg, input_view, input_add_on_appended_view},
        },

        //forms::checkbox::{CheckboxModel, checkbox_view}
    }
};

use std::rc::Rc;
use std::cell::RefCell;

// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    InputDefaultMsg(()),
    InputNormalLabelMsg(()),
    InputNormalFloatLabelMsg(()),
    InputFormLabelMsg(()),
    InputFormFloatLabelMsg(()),
    InputButtonMsg(InputButtonMsg),
    ButtonMsg(ButtonMsg),
    /*Checkbox1Msg(()),
    Checkbox2Msg(()),
    Checkbox3Msg(())*/
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    input_default: InputModel,
    input_normal_label: InputModel,
    input_normal_float_label: InputModel,
    input_button: InputModel,
    input_form_label: InputModel,
    input_form_float_label: InputModel,
    button: ButtonModel,
    /*checkbox1: CheckboxModel,
    checkbox2: CheckboxModel,
    checkbox3: CheckboxModel
    */
}


impl AppModel {
    pub fn new() -> Self {
        let mut input_default = InputModel::new();
        input_default.set_placeholder("input default");
        input_default.set_aria_label("Default input");

        let mut input_normal_label = InputModel::new();
        input_normal_label.set_input_id("normal_input_label");
        input_normal_label.set_placeholder("disabled normal input label");
        input_normal_label.set_label("Normal Input Label");
        input_normal_label.set_variant_label();
        input_normal_label.extend_label_classes(&["mr-1".to_owned()]);
        //input_normal_label.set_disabled();

        let mut input_normal_float_label = InputModel::new();
        input_normal_float_label.set_input_id("normal_input_float_label");
        input_normal_float_label.set_placeholder("normal input float label");
        input_normal_float_label.set_label("Normal Input Float Label");
        input_normal_float_label.set_variant_float_label();
        input_normal_float_label.set_validation_warn();
        input_normal_float_label.set_validation_warn_note("An Warn occured!!!");

        

        let mut input_form_label = InputModel::new();
        input_form_label.set_input_id("form_input_label");
        input_form_label.set_placeholder("Form Input Label");
        input_form_label.set_label("Form Input Label");
        input_form_label.set_variant_label();
        input_form_label.extend_label_classes(&["mr-1".to_owned()]);
        input_form_label.set_form_context();
        input_form_label.set_validation_errored();
        input_form_label.set_full_width();

        let mut input_form_float_label = InputModel::new();
        input_form_float_label.set_input_id("form_input_float_label");
        input_form_float_label.set_placeholder("Form Input Float Label");
        input_form_float_label.set_label("Form Input Float Label");
        input_form_float_label.set_variant_float_label();
        input_form_float_label.set_form_context();
        input_form_float_label.set_full_width();

        let mut input_button = InputModel::new();
        input_button.set_placeholder("input button");
        input_button.set_aria_label("Default input");

        
        let mut button = ButtonModel::new();
        button.set_variation_normal();
/*
        let mut checkbox1 = CheckboxModel::new();
        checkbox1.set_label("Available for hire");
        checkbox1.set_show_note();

        let mut checkbox2 = CheckboxModel::new();
        checkbox2.set_label("Available for hire 2");

        let mut checkbox3 = CheckboxModel::new();
        checkbox3.set_label("Available for hire 3");
        checkbox3.set_emphasized();
    */
        AppModel {
            input_default,
            input_normal_label,
            input_normal_float_label,
            input_form_label,
            input_form_float_label,
            input_button,
            button,
            /*checkbox1,
            checkbox2,
            checkbox3*/
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {
    div(
        &[],
        &[
            input_view(&app_model.input_default).map_msg(|msg| AppMsg::InputDefaultMsg(msg)),
            input_view(&app_model.input_normal_label).map_msg(|msg| AppMsg::InputNormalLabelMsg(msg)),
            input_view(&app_model.input_normal_float_label).map_msg(|msg| AppMsg::InputNormalFloatLabelMsg(msg)),
            
            input_add_on_appended_view(
                &app_model.input_button,
                button_view(
                    &app_model.button, 
                    text_view(&TextModel::new(), "B").map_msg(|_| ButtonMsg::NoAction),
                    None
                )
            ).map_msg(|msg| AppMsg::InputButtonMsg(msg)),
            input_view(&app_model.input_form_label).map_msg(|msg| AppMsg::InputFormLabelMsg(msg)),
            input_view(&app_model.input_form_float_label).map_msg(|msg| AppMsg::InputFormFloatLabelMsg(msg)),
            button_view(
                &app_model.button, 
                text_view(&TextModel::new(), "submit").map_msg(|_| ButtonMsg::NoAction),
                None
            ).map_msg(|btn_msg| AppMsg::ButtonMsg(btn_msg)),
            /*checkbox_view(
                &app_model.checkbox1,
                text::<()>(&[], "help-text-for-checkbox")
            ).map_msg(|msg| AppMsg::Checkbox1Msg(msg)),
            checkbox_view(
                &app_model.checkbox2,
                text::<()>(&[], "help-text-for-checkbox")
            ).map_msg(|msg| AppMsg::Checkbox2Msg(msg)),
            checkbox_view(
                &app_model.checkbox3,
                text::<()>(&[], "")
            ).map_msg(|msg| AppMsg::Checkbox3Msg(msg)),*/
        ]
    )
}



pub fn app_update(_model: &mut AppModel, _msg: AppMsg) -> Cmd<AppMsg> {
    Cmd::none()
}

pub fn app_subscription(_model: &AppModel) -> Sub<AppMsg> {
    Sub::none()
}
