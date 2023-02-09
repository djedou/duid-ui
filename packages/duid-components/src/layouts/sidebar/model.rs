/// Sidebar Model
#[derive(Debug, PartialEq, Clone)]
pub struct SidebarModel {
    pub(crate) root_classes: Vec<String>,
    pub(crate) root_style: Option<(&'static str, String)>,
    pub(crate) main_classes: Vec<String>,
    pub(crate) main_style: Option<(&'static str, String)>,
    pub(crate) sidebar_classes: Vec<String>,
    pub(crate) sidebar_style: Option<(&'static str, String)>,
    pub(crate) hide_mobile_sidebar: bool
}

impl SidebarModel {
    pub fn new() -> Self {
        SidebarModel {
            root_classes: Vec::with_capacity(0),
            root_style: None,
            main_classes: Vec::with_capacity(0),
            main_style: None,
            sidebar_classes: Vec::with_capacity(0),
            sidebar_style: None,
            hide_mobile_sidebar: true,
        }
    }

    pub fn extend_root_classes(&mut self, root_classes: &[String]) {
        let mut root_classes_clone = root_classes.to_owned();
        let mut root_classes_set = std::collections::HashSet::with_capacity(root_classes_clone.capacity());
        root_classes_clone.retain(|c| root_classes_set.insert(c.clone()));
        self.root_classes.extend_from_slice(&root_classes_clone);
    }

    pub fn remove_root_classes(&mut self, root_classes: &[String]) {
        self.root_classes.retain(|c| !root_classes.contains(c));
    }

    pub fn set_root_style(&mut self, root_style: Option<(&'static str, String)>) {
        self.root_style = root_style;
    }

    pub fn extend_main_classes(&mut self, main_classes: &[String]) {
        let mut main_classes_clone = main_classes.to_owned();
        let mut main_classes_set = std::collections::HashSet::with_capacity(main_classes_clone.capacity());
        main_classes_clone.retain(|c| main_classes_set.insert(c.clone()));
        self.main_classes.extend_from_slice(&main_classes_clone);
    }

    pub fn remove_main_classes(&mut self, main_classes: &[String]) {
        self.main_classes.retain(|c| !main_classes.contains(c));
    }

    pub fn set_main_style(&mut self, main_style: Option<(&'static str, String)>) {
        self.main_style = main_style;
    }

    pub fn extend_sidebar_classes(&mut self, sidebar_classes: &[String]) {
        let mut sidebar_classes_clone = sidebar_classes.to_owned();
        let mut sidebar_classes_set = std::collections::HashSet::with_capacity(sidebar_classes_clone.capacity());
        sidebar_classes_clone.retain(|c| sidebar_classes_set.insert(c.clone()));
        self.sidebar_classes.extend_from_slice(&sidebar_classes_clone);
    }

    pub fn remove_sidebar_classes(&mut self, sidebar_classes: &[String]) {
        self.sidebar_classes.retain(|c| !sidebar_classes.contains(c));
    }

    pub fn set_sidebar_style(&mut self, sidebar_style: Option<(&'static str, String)>) {
        self.sidebar_style = sidebar_style;
    }

    
    pub fn set_hide_mobile_sidebar(&mut self, value: bool) {
        self.hide_mobile_sidebar = value;
    }

    pub fn get_hide_mobile_sidebar(&mut self) -> bool {
        self.hide_mobile_sidebar
    }
}