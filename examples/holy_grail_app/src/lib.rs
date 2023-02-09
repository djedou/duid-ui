mod app;

use wasm_bindgen::prelude::*;
use duid_ui::{
    duid::apps::{UserApp, Duid},
    system::themes::{Theme, ThemeMode}
};
use app::{AppModel, AppMsg, app_view, app_update, app_subscription};

fn app() -> UserApp<AppModel, AppMsg> {
    UserApp::new(AppModel::new(), app_view, app_update, app_subscription)    
}


#[wasm_bindgen]
pub fn duid(node: &str) {
    let mut styles = Theme::new(ThemeMode::Light);
    
    Duid::start(
        &node, 
        app(),
        styles.get_base_themes(), // base_styles
        styles.get_themes(), // styles
    );
}