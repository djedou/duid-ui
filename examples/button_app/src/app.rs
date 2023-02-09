use duid_ui::{
    duid::{
        html::{
            div,
            nodes::Node,
            attributes::{classes, selectors},
        },
        duid_events::{NodeMapMsg, Cmd, Sub}
    },
    components::{
        typography::text::{text_view, TextModel},
        inputs::buttons::{ButtonModel, ButtonMsg, button_view}
    }
};

/*
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
*/

// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    Button(ButtonMsg),
    //ButtonGroup(ButtonGroupMsg)
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    button_text: TextModel,
    button_default_sm: ButtonModel,
    button_default_md: ButtonModel,
    button_default_large: ButtonModel,
    button_filled_sm_default: ButtonModel,
    button_filled_sm_danger: ButtonModel,
    button_filled_md: ButtonModel,
    button_filled_large: ButtonModel,
    button_outline_sm_default: ButtonModel,
    button_outline_sm_danger: ButtonModel,
    button_outline_md: ButtonModel,
    button_outline_large: ButtonModel,
    
    button_outline_block_md: ButtonModel,
    button_link_md: ButtonModel,
    button_outline_selected: ButtonModel,
    button_outline_disabled: ButtonModel,
    button_filled_selected: ButtonModel,
    button_filled_disabled: ButtonModel

    /*
    button_hidden_text_icon: ButtonModel,
    button_group_model: ButtonGroupModel,
    button_default_item_md: ButtonModel,
    button_filled_item_md: ButtonModel,
    button_outline_item_md: ButtonModel,
    button_danger_item_md: ButtonModel,*/
}


impl AppModel {
    pub fn new() -> Self {
        let button_text = TextModel::new();

        let mut button_default_sm = ButtonModel::new();
        button_default_sm.set_size_sm();
        button_default_sm.set_kind_default();
        button_default_sm.set_colors_default();
        let mut button_default_md = ButtonModel::new();
        button_default_md.set_kind_default();
        button_default_md.set_colors_default();
        let mut button_default_large = ButtonModel::new();
        button_default_large.set_size_large();
        button_default_large.set_kind_default();
        button_default_large.set_colors_default();

        let mut button_filled_sm_default = ButtonModel::new();
        button_filled_sm_default.set_size_sm();
        button_filled_sm_default.set_kind_filled();
        button_filled_sm_default.set_colors_filled_default();

        let mut button_filled_sm_danger = ButtonModel::new();
        button_filled_sm_danger.set_size_sm();
        button_filled_sm_danger.set_kind_filled();
        button_filled_sm_danger.set_colors_filled_danger();
        
        let mut button_filled_md = ButtonModel::new();
        button_filled_md.set_kind_filled();
        let mut button_filled_large = ButtonModel::new();
        button_filled_large.set_size_large();
        button_filled_large.set_kind_filled();

        let mut button_outline_sm_default = ButtonModel::new();
        button_outline_sm_default.set_size_sm();
        button_outline_sm_default.set_kind_outline();
        button_outline_sm_default.set_colors_outline_default();
        
        let mut button_outline_sm_danger = ButtonModel::new();
        button_outline_sm_danger.set_size_sm();
        button_outline_sm_danger.set_kind_outline();
        button_outline_sm_danger.set_colors_outline_danger();
        
        let mut button_outline_md = ButtonModel::new();
        button_outline_md.set_kind_outline();
        button_outline_md.set_colors_outline_default();
        let mut button_outline_large = ButtonModel::new();
        button_outline_large.set_size_large();
        button_outline_large.set_kind_outline();
        button_outline_large.set_colors_outline_default();

        /*
        let mut button_danger_sm = ButtonModel::new();
        let mut button_danger_md = ButtonModel::new();
        let mut button_danger_large = ButtonModel::new();
        button_danger_sm.set_size_sm();
        button_danger_sm.set_kind_danger();
        button_danger_md.set_kind_danger();
        button_danger_large.set_size_large();
        button_danger_large.set_kind_danger();
*/

        let mut button_outline_selected = ButtonModel::new();
        let mut button_outline_disabled = ButtonModel::new();
        button_outline_selected.set_kind_outline();
        button_outline_disabled.set_kind_outline();
        button_outline_selected.set_state_selected();
        button_outline_disabled.set_state_disabled();

        let mut button_filled_selected = ButtonModel::new();
        let mut button_filled_disabled = ButtonModel::new();
        button_filled_selected.set_kind_filled();
        button_filled_disabled.set_kind_filled();
        button_filled_selected.set_state_selected();
        button_filled_disabled.set_state_disabled();

        let mut button_link_md = ButtonModel::new();
        button_link_md.set_variation_link();
/*
        let mut button_invisible_md = ButtonModel::new();
        button_invisible_md.set_variation_invisible();

        let mut button_hidden_text_icon = ButtonModel::new();
        button_hidden_text_icon.set_button_hidden_text_icon();
*/
        let mut button_outline_block_md = ButtonModel::new();
        button_outline_block_md.set_kind_outline();
        button_outline_block_md.set_variation_block();
        button_outline_block_md.set_colors_outline_default();
/*
        let mut button_default_item_md = ButtonModel::new();
        button_default_item_md.set_group_item(true);
        let mut button_filled_item_md = ButtonModel::new();
        button_filled_item_md.set_group_item(true);
        button_filled_item_md.set_kind_filled();
        let mut button_outline_item_md = ButtonModel::new();
        button_outline_item_md.set_group_item(true);
        button_outline_item_md.set_kind_outline();
        let mut button_danger_item_md = ButtonModel::new();
        button_danger_item_md.set_group_item(true);
        button_danger_item_md.set_kind_danger();
        */
        
        AppModel {
            button_text,
            button_default_sm,
            button_default_md,
            button_default_large,
            button_filled_sm_default,
            button_filled_sm_danger,
            button_filled_md,
            button_filled_large,
            button_outline_sm_default,
            button_outline_sm_danger,
            button_outline_md,
            button_outline_large,
            button_outline_block_md,
            button_link_md,
            button_outline_selected,
            button_outline_disabled,
            button_filled_selected,
            button_filled_disabled,
            /*button_danger_sm,
            button_danger_md,
            button_danger_large,
            button_outline_selected_md,
            button_outline_disabled_md,
            button_invisible_md,
            button_hidden_text_icon,
            button_group_model: ButtonGroupModel::new(),
            button_default_item_md,
            button_filled_item_md,
            button_outline_item_md,
            button_danger_item_md*/
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {

    div(
        &[
            selectors(&[".btn-container:::mt-2 ml-2 flex gap-2 items-center".to_owned()])
        ],
        &[
            div(
                &[classes(&["btn-container".to_owned()])],
                &[
                    button_view(
                        &app_model.button_default_sm, 
                        text_view(&app_model.button_text, "default-sm").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_default_md,
                        text_view(&app_model.button_text, "default-md").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_default_large,
                        text_view(&app_model.button_text, "default-large").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg))
                ]
            ),
            div(
                &[classes(&["btn-container".to_owned()])],
                &[
                    button_view(
                        &app_model.button_filled_sm_default, 
                        text_view(&app_model.button_text, "filled-sm-default").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_filled_sm_danger, 
                        text_view(&app_model.button_text, "filled-sm-danger").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_filled_md,
                        text_view(&app_model.button_text, "filled-md").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_filled_large,
                        text_view(&app_model.button_text, "filled-large").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg))
                ]
            ),
            div(
                &[classes(&["btn-container".to_owned()])],
                &[
                    button_view(
                        &app_model.button_outline_sm_default, 
                        text_view(&app_model.button_text, "outline-sm-default").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_outline_sm_danger, 
                        text_view(&app_model.button_text, "outline-sm-danger").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_outline_md,
                        text_view(&app_model.button_text, "outline-md").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_outline_large,
                        text_view(&app_model.button_text, "outline-large").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg))
                ]
            ),
            /*div(
                &[classes(&["btn-container".to_owned()])],
                &[
                    button_view(
                        &app_model.button_danger_sm, 
                        text("danger-sm"),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_danger_md, 
                        text("danger-md"),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_danger_large, 
                        text("danger-large"),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg))
                ]
            ),*/
            div(
                &[classes(&["btn-container".to_owned()])],
                &[
                    button_view(
                        &app_model.button_outline_selected,
                        text_view(&app_model.button_text, "outline-selected-md").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_outline_disabled,
                        text_view(&app_model.button_text, "outline-disabled-md").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_filled_selected,
                        text_view(&app_model.button_text, "filled-selected-md").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_filled_disabled,
                        text_view(&app_model.button_text, "filled-disabled-md").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                    button_view(
                        &app_model.button_link_md,
                        text_view(&app_model.button_text, "button-link-md").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg))
                ]
            ),
            div(
                &[classes(&["btn-container".to_owned()])],
                &[
                    button_view(
                        &app_model.button_outline_block_md,
                        text_view(&app_model.button_text, "button-outline-block-md").map_msg(|_| ButtonMsg::NoAction),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg)),
                ]
            ),
            /*div(
                &[classes(&["btn-container".to_owned()])],
                &[
                    button_view(
                        &app_model.button_hidden_text_icon, 
                        text("button-hidden-text-icon"),
                        None
                    ).map_msg(|msg| AppMsg::Button(msg))
                ]
            ),
            div(
                &[classes(&["btn-container".to_owned()])],
                &[
                    button_group_view(
                        &app_model.button_group_model, 
                        vec![
                            (
                                1,
                                button_view(
                                    &app_model.button_outline_item_md, 
                                    text("outline-md"),
                                    None
                                )
                            ),
                            (
                                2,
                                button_view(
                                    &app_model.button_default_item_md, 
                                    text("default-md"),
                                    None
                                )
                            ),
                            (
                                3,
                                button_view(
                                    &app_model.button_filled_item_md, 
                                    text("filled-md"),
                                    None
                                )
                            ),
                            (
                                4,
                                button_view(
                                    &app_model.button_danger_item_md, 
                                    text("danger-md"),
                                    None
                                )
                            )
                        ]
                    ).map_msg(|msg| AppMsg::ButtonGroup(msg))
                ]
            )*/
        
        ]
    )
}


pub fn app_update(_model: &mut AppModel, _msg: AppMsg) -> Cmd<AppMsg> {
    Cmd::none()
}

pub fn app_subscription(_model: &AppModel) -> Sub<AppMsg> {
    Sub::none()
}