use super::{TextModel, TextTag, TextMsg};
use duid::{
        html::{
            div, span, text, i, em, strong, mark, cite, dfn,
            attributes::{classes, selectors},
            nodes::Node
        }
};

/// text view to render a text
pub fn text_view(text_model: &TextModel, child: impl AsRef<str>) -> Node<TextMsg> {
    let text_classes: Vec<_> = text_model.classes.iter().collect();
    let text_selectors: Vec<_> = text_model.selectors.iter().collect();
    
    let mut attrs = vec![classes(&text_classes), selectors(&text_selectors)];
    attrs.extend_from_slice(&text_model.attributes);
    
    match text_model.tag {
        TextTag::Span => {
            span(
                &attrs,
                &[
                    text(child.as_ref())
                ]
            )
        },
        TextTag::Div => {
            div(
                &attrs,
                &[
                    text(child.as_ref())
                ]
            )
        },
        TextTag::Italic => {
            i(
                &attrs,
                &[
                    text(child.as_ref())
                ]
            )
        },
        TextTag::Em => {
            em(
                &attrs,
                &[
                    text(child.as_ref())
                ]
            )
        },
        TextTag::Strong => {
            strong(
                &attrs,
                &[
                    text(child.as_ref())
                ]
            )
        },
        TextTag::Mark => {
            mark(
                &attrs,
                &[
                    text(child.as_ref())
                ]
            )
        },
        TextTag::Cite => {
            cite(
                &attrs,
                &[
                    text(child.as_ref())
                ]
            )
        },
        TextTag::Dfn => {
            dfn(
                &attrs,
                &[
                    text(child.as_ref())
                ]
            )
        }
    }
}
