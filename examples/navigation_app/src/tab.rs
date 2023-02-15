use duid_ui::{
    duid::{
        html::{
            text, div,
            nodes::Node,
        },
        duid_events::{NodeMapMsg, Cmd, Sub}
    },
    components::{
        navigation::{
            //page::{page_view, PageModel, PageMsg}
            tab_nav::{TabNavMsg, TabNavModel, tab_nav_view, tab_nav_element_view, TabNavItemMsg, TabNavElementModel}
        },
        typography::{
            text::{text_view, TextModel},
        }
    }
};


// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum Messages {
    Msg,
}


// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum TabAppMsg {
    Msg(TabNavMsg<Messages>)
}

#[derive(Debug, Clone, PartialEq)]
pub struct TabAppModel {
    pub tab_nav: TabNavModel,
    pub tab_index: usize,
    pub menus: Vec<TabNavElementModel>

}


impl TabAppModel {
    pub fn new() -> Self {
        let mut default_menu = TabNavElementModel::new();
        default_menu.set_selected(true);

        let menus = vec![
            default_menu,
            TabNavElementModel::new(),
            TabNavElementModel::new()
        ];

        TabAppModel {
            tab_nav: TabNavModel::new(),
            tab_index: 0,
            menus
        }
    }
}

pub fn tab_app_view(tab_app_model: &TabAppModel) -> Node<TabAppMsg> {

    let menu_items: Vec<_> = tab_app_model.menus.iter().enumerate().zip(&["Djed", "Arnaud", "Aman"])
        .map(|((index, model), info)| 
            (
                index, 
                tab_nav_element_view(
                    &model, 
                    &[text_view(&TextModel::new(), info).map_msg(|_| TabNavItemMsg::NoAction)]
                )
            )
        )
        .collect();


    tab_nav_view(
        &tab_app_model.tab_nav,
        div(
            &[], 
            &[
                match tab_app_model.tab_index {
                    1 => text_view(&TextModel::new(), "Content 2").map_msg(|_| TabNavMsg::Content(Messages::Msg)),
                    2 => text_view(&TextModel::new(), "Content 3").map_msg(|_| TabNavMsg::Content(Messages::Msg)),
                    _ => text_view(&TextModel::new(), "Content 1").map_msg(|_| TabNavMsg::Content(Messages::Msg))
                }
                
            ]
        ),
        menu_items,
        None//vec![(1, div(&[], &[]))]
    ).map_msg(|m| TabAppMsg::Msg(m))
}


pub fn tab_app_update(model: &mut TabAppModel, msg: TabAppMsg) -> Cmd<TabAppMsg> {
    match msg {
        TabAppMsg::Msg(tab) => {
            match tab {
                TabNavMsg::Item((index, m)) => {
                    model.menus[index].set_selected(true);
                    model.menus[model.tab_index].set_selected(false);
                    model.tab_index = index;

                    //duid_ui::duid::console::info!("tab index: {} message: {:#?}", index, m);
                    Cmd::none()
                },
                _ => Cmd::none()
            }
            
        }
    }
}

pub fn tab_app_subscription(_model: &TabAppModel) -> Sub<TabAppMsg> {
    Sub::none()
}