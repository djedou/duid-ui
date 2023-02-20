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
            tab_nav::{TabNavMsg, TabNavModel, tab_nav_view, tab_nav_element_view, TabNavItemMsg, TabNavElementModel}
        },
        typography::{
            text::{text_view, TextModel},
        }
    }
};



// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum TabAppMsg {
    Msg(TabNavMsg)
}

#[derive(Debug, Clone, PartialEq)]
pub struct TabAppModel {
    pub tab_nav: TabNavModel,
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
        menu_items,
        None//vec![(1, div(&[], &[]))]
    ).map_msg(|m| TabAppMsg::Msg(m))
}


pub fn tab_app_update(model: &mut TabAppModel, msg: TabAppMsg) -> Cmd<TabAppMsg> {
    match msg {
        TabAppMsg::Msg(tab) => {
            match tab {
                TabNavMsg::Item((index, m)) => {
                    model.menus.iter_mut().enumerate().for_each(|(i,item)| {
                        if index == i {
                            item.set_selected(true);
                        }
                        else {
                            item.set_selected(false);
                        }
                    });

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