use super::{ButtonIcon, ButtonKind, ButtonSize, ButtonState, ButtonVariation, ButtonColor, ButtonMsg, get_button_default_selectors};
use std::collections::{HashSet, HashMap};
use duid::html::attributes::Attribute;


/// Button Model
#[derive(Debug, PartialEq, Clone)]
pub struct ButtonModel {
    pub(crate) selectors: HashSet<String>,
    pub(crate) classes: HashSet<String>,
    pub(crate) attributes: Vec<Attribute<ButtonMsg>>,
    pub(crate) icon_classes: HashMap<ButtonIcon, HashSet<String>>,
    pub(crate) icon: ButtonIcon,
    pub(crate) kind_classes: HashMap<ButtonKind, HashSet<String>>,
    pub(crate) kind: ButtonKind,
    pub(crate) size_classes: HashMap<ButtonSize, HashSet<String>>,
    pub(crate) size: ButtonSize,
    pub(crate) state_classes: HashMap<ButtonState, HashSet<String>>,
    pub(crate) state: ButtonState,
    pub(crate) variation_classes: HashMap<ButtonVariation, HashSet<String>>,
    pub(crate) variation: ButtonVariation,
    pub(crate) colors_classes: HashMap<ButtonColor, HashSet<String>>,
    pub(crate) colors: ButtonColor,
    pub(crate) count: usize,
    pub(crate) is_group_item: bool
}

// ButtonVariation => ButtonState
// ButtonKind => ButtonState
impl ButtonModel {
    pub fn new() -> ButtonModel {
      let mut kind_classes = HashMap::with_capacity(0);
      let mut colors_classes = HashMap::with_capacity(0);
      let mut size_classes = HashMap::with_capacity(0);
      let mut icon_classes = HashMap::with_capacity(0);
      let mut variation_classes = HashMap::with_capacity(0);
      
      let mut classes = HashSet::with_capacity(0);
      classes.insert("btn".to_owned());

      // ButtonKind::Default
      let mut kind_default_classes_set = HashSet::with_capacity(0);
      let _ = kind_default_classes_set.insert("btn-default".to_owned());
      let _ = kind_classes.insert(ButtonKind::Default, kind_default_classes_set);
      
      // ButtonKind::Filled
      let mut kind_filled_classes_set = HashSet::with_capacity(0);
      let _ = kind_filled_classes_set.insert("btn-filled".to_owned());
      let _ = kind_classes.insert(ButtonKind::Filled, kind_filled_classes_set);

      // ButtonColor::Filled Color Default
      let mut colors_filled_default_classes_set = HashSet::with_capacity(0);
      let _ = colors_filled_default_classes_set.insert("btn-filled-colors".to_owned());
      let _ = colors_classes.insert(ButtonColor::FilledDefault, colors_filled_default_classes_set);
      
      // ButtonColor::Filled Color Danger
      let mut colors_filled_danger_classes_set = HashSet::with_capacity(0);
      let _ = colors_filled_danger_classes_set.insert("btn-filled-danger-colors".to_owned());
      let _ = colors_classes.insert(ButtonColor::FilledDanger, colors_filled_danger_classes_set);
      

      // ButtonKind::Outline
      let mut kind_outline_classes_set = HashSet::with_capacity(0);
      let _ = kind_outline_classes_set.insert("btn-outline".to_owned());
      let _ = kind_classes.insert(ButtonKind::Outline, kind_outline_classes_set);

      // ButtonColor::Outline Color Default
      let mut colors_outline_default_classes_set = HashSet::with_capacity(0);
      let _ = colors_outline_default_classes_set.insert("btn-outline-colors".to_owned());
      let _ = colors_classes.insert(ButtonColor::OutlineDefault, colors_outline_default_classes_set);


      // ButtonColor::Outline Color Danger
      let mut colors_outline_danger_classes_set = HashSet::with_capacity(0);
      let _ = colors_outline_danger_classes_set.insert("btn-outline-colors-danger".to_owned());
      let _ = colors_classes.insert(ButtonColor::OutlineDanger, colors_outline_danger_classes_set);
      
      // Button Size sm
      let mut button_sm_classes_set = HashSet::with_capacity(0);
      let _ = button_sm_classes_set.insert("btn-sm".to_owned());
      let _ = size_classes.insert(ButtonSize::Small, button_sm_classes_set);
      
      // Button Size md
      let mut button_md_classes_set = HashSet::with_capacity(0);
      let _ = button_md_classes_set.insert("btn-md".to_owned());
      let _ = size_classes.insert(ButtonSize::Medium, button_md_classes_set);
      
      // Button Size lg
      let mut button_lg_classes_set = HashSet::with_capacity(0);
      let _ = button_lg_classes_set.insert("btn-lg".to_owned());
      let _ = size_classes.insert(ButtonSize::Large, button_lg_classes_set);
      

      // Button Icon Only
      let mut button_icon_only_classes_set = HashSet::with_capacity(0);
      let _ = button_icon_only_classes_set.insert("btn-octicon".to_owned());
      let _ = icon_classes.insert(ButtonIcon::Only, button_icon_only_classes_set);


      // Button Icon Close
      let mut button_icon_close_classes_set = HashSet::with_capacity(0);
      let _ = button_icon_close_classes_set.insert("close-button".to_owned());
      let _ = icon_classes.insert(ButtonIcon::Close, button_icon_close_classes_set);
      

      // Button Icon HiddenText
      let mut button_icon_expander_classes_set = HashSet::with_capacity(0);
      let _ = button_icon_expander_classes_set.insert("ellipsis-expander".to_owned());
      let _ = icon_classes.insert(ButtonIcon::HiddenText, button_icon_expander_classes_set);
      
      // Button Icon Count
      let mut button_icon_count_classes_set = HashSet::with_capacity(0);
      let _ = button_icon_count_classes_set.insert("btn-with-count social-count".to_owned());
      let _ = icon_classes.insert(ButtonIcon::Count, button_icon_count_classes_set);
      
      // Button Variaton Normal
      let mut variation_normal_classes_set = HashSet::with_capacity(0);
      let _ = variation_normal_classes_set.insert("btn-normal".to_owned());
      let _ = variation_classes.insert(ButtonVariation::Normal, variation_normal_classes_set);
      

      // Button Variaton Block
      let mut variation_block_classes_set = HashSet::with_capacity(0);
      let _ = variation_block_classes_set.insert("btn-block".to_owned());
      let _ = variation_classes.insert(ButtonVariation::Block, variation_block_classes_set);
      
      // Button Variaton Link
      let mut variation_link_classes_set = HashSet::with_capacity(0);
      let _ = variation_link_classes_set.insert("btn-link".to_owned());
      let _ = variation_classes.insert(ButtonVariation::Link, variation_link_classes_set);
      
      // Button Variaton Summary
      let mut variation_summary_classes_set = HashSet::with_capacity(0);
      let _ = variation_summary_classes_set.insert("btn-normal".to_owned());
      let _ = variation_classes.insert(ButtonVariation::Summary, variation_summary_classes_set);
      

      ButtonModel {
          selectors: get_button_default_selectors(),
          classes,
          attributes: Vec::with_capacity(0),
          icon_classes,
          icon: ButtonIcon::None,
          kind_classes,
          kind: ButtonKind::Default,
          size_classes,
          size: ButtonSize::Medium,
          state_classes: HashMap::with_capacity(0),
          state: ButtonState::Normal,
          variation_classes,
          variation: ButtonVariation::Normal,
          colors_classes,
          colors: ButtonColor::FilledDefault,
          count: 0,
          is_group_item: false
      }
    }
    
    pub fn add_attributes(&mut self, attributes: &[Attribute<ButtonMsg>]) {
      self.attributes.extend_from_slice(attributes);
    }

    pub fn add_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.classes.insert(c.as_ref().to_owned());
        });
    }

    pub fn add_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.selectors.insert(c.as_ref().to_owned());
        });
    }

    pub fn remove_classes(&mut self, classes: &[impl AsRef<str>]) {
        classes.iter().for_each(|c| {
            let _ = self.classes.remove(c.as_ref());
        });
    }

    pub fn remove_selectors(&mut self, selectors: &[impl AsRef<str>]) {
        selectors.iter().for_each(|c| {
            let _ = self.selectors.remove(c.as_ref());
        });
    }

    pub fn set_size_sm(&mut self) {
        self.size = ButtonSize::Small;
    }

    pub fn set_size_md(&mut self) {
        self.size = ButtonSize::Medium;
    }

    pub fn set_size_large(&mut self) {
        self.size = ButtonSize::Large;
    }

    pub fn set_kind_default(&mut self) {
        self.kind = ButtonKind::Default;
    }

    pub fn set_kind_filled(&mut self) {
        self.kind = ButtonKind::Filled;
    }

    pub fn set_kind_outline(&mut self) {
        self.kind = ButtonKind::Outline;
    }

    pub fn set_colors_default(&mut self) {
      self.colors = ButtonColor::Default;
    }

	pub fn set_colors_filled_default(&mut self) {
		self.colors = ButtonColor::FilledDefault;
	}
	
    pub fn set_colors_outline_default(&mut self) {
      self.colors = ButtonColor::OutlineDefault;
    }
    
    pub fn set_colors_filled_danger(&mut self) {
      self.colors = ButtonColor::FilledDanger;
    }
    
    pub fn set_colors_outline_danger(&mut self) {
      self.colors = ButtonColor::OutlineDanger;
    }

    pub fn set_state_normal(&mut self) {
        self.state = ButtonState::Normal;
    }

    pub fn set_state_selected(&mut self) {
        self.state = ButtonState::Selected;
    }

    pub fn set_state_disabled(&mut self) {
        self.state = ButtonState::Disabled;
    }

    pub fn set_variation_normal(&mut self) {
        self.variation = ButtonVariation::Normal;
    }

    pub fn set_variation_block(&mut self) {
        self.variation = ButtonVariation::Block;
    }

    pub fn set_variation_link(&mut self) {
        self.variation = ButtonVariation::Link;
    }

    pub fn set_variation_summary(&mut self) {
        self.variation = ButtonVariation::Summary;
    }
    

    pub fn set_button_icon_none(&mut self) {
        self.icon = ButtonIcon::None;
    }

    pub fn set_button_with_icon(&mut self) {
        self.icon = ButtonIcon::With;
    }

    pub fn set_button_only_icon(&mut self) {
        self.icon = ButtonIcon::Only;
    }

    pub fn set_button_close_icon(&mut self) {
        self.icon = ButtonIcon::Close;
    }

    pub fn set_button_count_icon(&mut self) {
        self.icon = ButtonIcon::Count;
    }

    pub fn set_button_hidden_text_icon(&mut self) {
        self.icon = ButtonIcon::HiddenText;
    }

    pub fn set_count(&mut self, count: usize) {
        self.count = count;
    }

    pub fn set_group_item(&mut self, item: bool) {
        self.is_group_item = item;
    }
}