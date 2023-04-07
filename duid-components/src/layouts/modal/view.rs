use super::{ModalMsg, ModalModel};
use crate::{
    inputs::buttons::{ButtonMsg, button_view},
    typography::text::text_view
};
use duid::{
        html::{
            details, div,
            attributes::{classes, selectors, open},
            nodes::Node
        },
        duid_events::{NodeMapMsg}    
};


pub fn modal_view<M: Clone + 'static>(
    modal_model: &ModalModel,
    content: Node<M>,
) -> Node<ModalMsg<M>> {
    
    let modal_classes: Vec<_> = modal_model.classes.iter().map(|c| c.to_owned()).collect();
    let modal_selectors: Vec<_> = modal_model.selectors.iter().collect();
    
    details(
        &[
            classes(&modal_classes),
            selectors(&modal_selectors),
            open(!modal_model.is_closed)
        ],
        &[
            button_view(
                &modal_model.button_model,
                text_view(
                    &modal_model.button_text_model, 
                    &modal_model.button_text
                ).map_msg(|_| ButtonMsg::NoAction),
                None
            ).map_msg(|m| ModalMsg::Button(m)),
            content.map_msg(|m| ModalMsg::Msg(m))
        ]
    )
}
