use super::{PageModel, PageMsg, PageElement};
use std::collections::HashSet;
use duid::{
        html::{
            div, header, main, footer,
            attributes::{classes, selectors},
            nodes::Node
        },
        duid_events::NodeMapMsg
};

/// page view 
pub fn page_view<M>(
    page_model: &PageModel, 
    header_view: Option<Node<M>>, 
    body_view: Node<M>,
    footer_view: Option<Node<M>>
) -> Node<PageMsg<M>>
where 
    M: Clone + 'static
{
    
    let page_classes: Vec<_> = page_model.classes.iter().collect();
    let page_selectors: Vec<_> = page_model.selectors.iter().collect();
    let attrs = vec![classes(&page_classes), selectors(&page_selectors)];

    match (header_view, footer_view) {
        (Some(header), Some(footer)) => {
            div(
                &attrs,
                &[
                    build_child::<M>(
                        &page_model.header_classes, 
                        &page_model.header_selectors, 
                        header,
                        &page_model.header_height,
                        &page_model.footer_height,
                        PageElement::Header
                    ).map_msg(|m| PageMsg::Element(m)),
                    build_child::<M>(
                        &page_model.body_classes, 
                        &page_model.body_selectors,
                        body_view,
                        &page_model.header_height,
                        &page_model.footer_height,
                        PageElement::Body
                    ).map_msg(|m| PageMsg::Element(m)),
                    build_child::<M>(
                        &page_model.footer_classes, 
                        &page_model.footer_selectors, 
                        footer,
                        &page_model.header_height,
                        &page_model.footer_height,
                        PageElement::Footer
                    ).map_msg(|m| PageMsg::Element(m))
                ]
            )
        },
        (None, Some(footer)) => {
            div(
                &attrs,
                &[
                    build_child::<M>(
                        &page_model.body_classes, 
                        &page_model.body_selectors, 
                        body_view,
                        "0rem",
                        &page_model.footer_height,
                        PageElement::Body
                    ).map_msg(|m| PageMsg::Element(m)),
                    build_child::<M>(
                        &page_model.footer_classes, 
                        &page_model.footer_selectors, 
                        footer,
                        "0rem",
                        &page_model.footer_height,
                        PageElement::Footer
                    ).map_msg(|m| PageMsg::Element(m))
                ]
            )
        },
        (Some(header), None) => {
            div(
                &attrs,
                &[
                    build_child::<M>(
                        &page_model.header_classes, 
                        &page_model.header_selectors, 
                        header,
                        &page_model.header_height,
                        "0rem",
                        PageElement::Header
                    ).map_msg(|m| PageMsg::Element(m)),
                    build_child::<M>(
                        &page_model.body_classes, 
                        &page_model.body_selectors, 
                        body_view,
                        &page_model.header_height,
                        "0rem",
                        PageElement::Body
                    ).map_msg(|m| PageMsg::Element(m))
                ]
            )
        },
        (None, None) => {
            div(
                &attrs,
                &[
                    build_child::<M>(
                        &page_model.body_classes, 
                        &page_model.body_selectors, 
                        body_view,
                        "0rem",
                        "0rem",
                        PageElement::Body
                    ).map_msg(|m| PageMsg::Element(m))
                ]
            )
        }
    }
}

fn build_child<M>(
    child_classes: &HashSet<String>, 
    child_selectors: &HashSet<String>, 
    child: Node<M>, 
    header_height: &str, 
    footer_height: &str, 
    elm: PageElement
) -> Node<M> 
where 
    M: Clone + 'static
{
    let (child_height_classes, child_height_selectors) = render_heights(&header_height, &footer_height, &elm);
    let mut classes_vec: Vec<_> = child_classes.iter().collect();
    let mut selectors_vec: Vec<_> = child_selectors.iter().collect();
    classes_vec.push(&child_height_classes);
    selectors_vec.push(&child_height_selectors);


    let attrs = vec![classes(&classes_vec), selectors(&selectors_vec)];

    match elm {
        PageElement::Header => {
            header(
                &attrs,
                &[child]
            )
        },
        PageElement::Body => {
            main(
                &attrs,
                &[child]
            )
        },
        PageElement::Footer => {
            footer(
                &attrs,
                &[child]
            )
        }
    }
}

fn render_heights(header_height: &str, footer_height: &str, elm: &PageElement) -> (String, String) {
    match elm {
        PageElement::Header => {
            let header_height_selector = format!(".duid-ph-height-{}:::vh-90[{}]", header_height.replace(".", ""), header_height);
            let header_height_classe = format!("duid-ph-height-{}", header_height.replace(".", ""));
            (header_height_classe, header_height_selector)
        },
        PageElement::Body => {
            let body_height_selector = format!(".duid-pb-height-{}-{}:::vh-90[calc(99.6vh&-&{}&-&{})]", header_height.replace(".", ""), footer_height.replace(".", ""), header_height, footer_height);
            let body_height_classe = format!("duid-pb-height-{}-{}", header_height.replace(".", ""), footer_height.replace(".", ""));
            (body_height_classe, body_height_selector)
        },
        PageElement::Footer => {
            let footer_height_selector = format!(".duid-pf-height-{}:::vh-90[{}]", footer_height.replace(".", ""), footer_height);
            let footer_height_classe = format!("duid-pf-height-{}", footer_height.replace(".", ""));
            (footer_height_classe, footer_height_selector)
        }
    }
}