use super::{AutoCompleteMsg, AutoCompleteModel};
use crate::{
    typography::text::text_view
};
use duid::{
        html::{
            input, div,
            attributes::{classes, selectors, Attribute, AttributeValue, Value},
            nodes::Node
        },
        duid_events::{NodeMapMsg}    
};


pub fn auto_complete_view(auto_complete_model: &AutoCompleteModel) -> Node<AutoCompleteMsg> {
    div(
        &[
            //classes(&["SelectMenu-item".to_owned()]),
        ],
        &[
            //text_view(&TextModel::new(), "Item 1").map_msg(|_| Messages::NoAction),
            input(
                &[
                    Attribute::new(None, "type", AttributeValue::from_value(Value::String("search".to_string())))
                ], 
                &[]
            )
        ]
    )
}