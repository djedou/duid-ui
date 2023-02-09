use duid_system::heights::Height;

/// HolyGrail Model
#[derive(Debug, PartialEq, Clone)]
pub struct HolyGrailModel {
    pub(crate) holy_grail_root_classes: Vec<String>,
    pub(crate) holy_grail_root_style: Option<(&'static str, String)>,
    pub(crate) header_classes: Vec<String>,
    pub(crate) header_style: Option<(&'static str, String)>,
    pub(crate) left_sidebar_classes: Vec<String>,
    pub(crate) left_sidebar_style: Option<(&'static str, String)>,
    pub(crate) body_classes: Vec<String>,
    pub(crate) body_style: Option<(&'static str, String)>,
    pub(crate) right_sidebar_classes: Vec<String>,
    pub(crate) right_sidebar_style: Option<(&'static str, String)>,
    pub(crate) footer_classes: Vec<String>,
    pub(crate) footer_style: Option<(&'static str, String)>,
    pub(crate) hide_mobile_left_sidebar: bool,
    pub(crate) hide_mobile_right_sidebar: bool,
    pub(crate) header_height: Height,
    pub(crate) footer_height: Height
}

impl HolyGrailModel {
    pub fn new() -> Self {
        HolyGrailModel {
            holy_grail_root_classes: Vec::with_capacity(0),
            holy_grail_root_style: None,
            header_classes: Vec::with_capacity(0),
            header_style: None,
            left_sidebar_classes: Vec::with_capacity(0),
            left_sidebar_style: None,
            body_classes: Vec::with_capacity(0),
            body_style: None,
            right_sidebar_classes: Vec::with_capacity(0),
            right_sidebar_style: None,
            footer_classes: Vec::with_capacity(0),
            footer_style: None,
            hide_mobile_left_sidebar: true,
            hide_mobile_right_sidebar: true,
            header_height: Height::H2(None),
            footer_height: Height::H2(None)
        }
    }

    pub fn extend_holy_grail_root_classes(&mut self, holy_grail_root_classes: &[String]) {
        let mut holy_grail_root_classes_clone = holy_grail_root_classes.to_owned();
        let mut holy_grail_root_classes_set = std::collections::HashSet::with_capacity(holy_grail_root_classes_clone.capacity());
        holy_grail_root_classes_clone.retain(|c| holy_grail_root_classes_set.insert(c.clone()));
        self.holy_grail_root_classes.extend_from_slice(&holy_grail_root_classes_clone);
    }

    pub fn remove_holy_grail_root_classes(&mut self, holy_grail_root_classes: &[String]) {
        self.holy_grail_root_classes.retain(|c| !holy_grail_root_classes.contains(c));
    }

    pub fn set_holy_grail_root_style(&mut self, holy_grail_root_style: Option<(&'static str, String)>) {
        self.holy_grail_root_style = holy_grail_root_style;
    }

    pub fn extend_header_classes(&mut self, header_classes: &[String]) {
        let mut header_classes_clone = header_classes.to_owned();
        let mut header_classes_set = std::collections::HashSet::with_capacity(header_classes_clone.capacity());
        header_classes_clone.retain(|c| header_classes_set.insert(c.clone()));
        self.header_classes.extend_from_slice(&header_classes_clone);
    }

    pub fn remove_header_classes(&mut self, header_classes: &[String]) {
        self.header_classes.retain(|c| !header_classes.contains(c));
    }

    pub fn set_header_style(&mut self, header_style: Option<(&'static str, String)>) {
        self.header_style = header_style;
    }

    pub fn extend_left_sidebar_classes(&mut self, left_sidebar_classes: &[String]) {
        let mut left_sidebar_classes_clone = left_sidebar_classes.to_owned();
        let mut left_sidebar_classes_set = std::collections::HashSet::with_capacity(left_sidebar_classes_clone.capacity());
        left_sidebar_classes_clone.retain(|c| left_sidebar_classes_set.insert(c.clone()));
        self.left_sidebar_classes.extend_from_slice(&left_sidebar_classes_clone);
    }

    pub fn remove_left_sidebar_classes(&mut self, left_sidebar_classes: &[String]) {
        self.left_sidebar_classes.retain(|c| !left_sidebar_classes.contains(c));
    }

    pub fn set_left_sidebar_style(&mut self, left_sidebar_style: Option<(&'static str, String)>) {
        self.left_sidebar_style = left_sidebar_style;
    }

    pub fn set_footer_height(&mut self, footer_height: Height) {
        self.footer_height = footer_height;
    }

    pub fn set_header_height(&mut self, header_height: Height) {
        self.header_height = header_height;
    }

    pub fn extend_body_classes(&mut self, body_classes: &[String]) {
        let mut body_classes_clone = body_classes.to_owned();
        let mut body_classes_set = std::collections::HashSet::with_capacity(body_classes_clone.capacity());
        body_classes_clone.retain(|c| body_classes_set.insert(c.clone()));
        self.body_classes.extend_from_slice(&body_classes_clone);
    }

    pub fn remove_body_classes(&mut self, body_classes: &[String]) {
        self.body_classes.retain(|c| !body_classes.contains(c));
    }

    pub fn set_body_style(&mut self, body_style: Option<(&'static str, String)>) {
        self.body_style = body_style;
    }

    pub fn extend_right_sidebar_classes(&mut self, right_sidebar_classes: &[String]) {
        let mut right_sidebar_classes_clone = right_sidebar_classes.to_owned();
        let mut right_sidebar_classes_set = std::collections::HashSet::with_capacity(right_sidebar_classes_clone.capacity());
        right_sidebar_classes_clone.retain(|c| right_sidebar_classes_set.insert(c.clone()));
        self.right_sidebar_classes.extend_from_slice(&right_sidebar_classes_clone);
    }

    pub fn remove_right_sidebar_classes(&mut self, right_sidebar_classes: &[String]) {
        self.right_sidebar_classes.retain(|c| !right_sidebar_classes.contains(c));
    }

    pub fn set_right_sidebar_style(&mut self, right_sidebar_style: Option<(&'static str, String)>) {
        self.right_sidebar_style = right_sidebar_style;
    }

    pub fn extend_footer_classes(&mut self, footer_classes: &[String]) {
        let mut footer_classes_clone = footer_classes.to_owned();
        let mut footer_classes_set = std::collections::HashSet::with_capacity(footer_classes_clone.capacity());
        footer_classes_clone.retain(|c| footer_classes_set.insert(c.clone()));
        self.footer_classes.extend_from_slice(&footer_classes_clone);
    }

    pub fn remove_footer_classes(&mut self, footer_classes: &[String]) {
        self.footer_classes.retain(|c| !footer_classes.contains(c));
    }

    pub fn set_footer_style(&mut self, footer_style: Option<(&'static str, String)>) {
        self.footer_style = footer_style;
    }

    pub fn set_hide_mobile_left_sidebar(&mut self, value: bool) {
        self.hide_mobile_left_sidebar = value;
    }

    pub fn get_hide_mobile_left_sidebar(&mut self) -> bool {
        self.hide_mobile_left_sidebar
    }

    pub fn set_hide_mobile_right_sidebar(&mut self, value: bool) {
        self.hide_mobile_right_sidebar = value;
    }

    pub fn get_hide_mobile_right_sidebar(&mut self) -> bool {
        self.hide_mobile_right_sidebar
    }
}