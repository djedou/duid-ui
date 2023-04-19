use super::{ModalMsg, ModalModel};
use duid::{
        html::{
            details,
            attributes::{classes, selectors, open},
            nodes::Node
        },
        duid_events::{NodeMapMsg}    
};


pub fn modal_view<M: Clone + 'static>(
    button_view: Node<M>,
    modal_model: &ModalModel,
    content: Node<M>,
) -> Node<ModalMsg<M>> {
    
    let modal_classes: Vec<_> = modal_model.classes.iter().map(|c| c.to_owned()).collect();
    let modal_selectors: Vec<_> = modal_model.selectors.iter().collect();
    
    details(
        &[
            classes(&modal_classes),
            selectors(&modal_selectors),
            open(!modal_model.is_closed())
        ],
        &[
            button_view.map_msg(|m| ModalMsg::Msg(m)),
            content.map_msg(|m| ModalMsg::Msg(m))
        ]
    )
}
