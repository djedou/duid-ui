use super::{SelectMenuMsg, SelectMenuModel};
use crate::{
    inputs::buttons::{ButtonMsg, button_view},
    typography::text::text_view
};
use duid::{
        html::{
            div,
            attributes::{classes, selectors, Attribute, AttributeValue, Value},
            nodes::Node
        },
        duid_events::{NodeMapMsg}    
};


pub fn select_menu_view<M: Clone + 'static>(
    select_menu_model: &SelectMenuModel,
    header: Option<Node<M>>, 
    body: Node<M>, 
    footer: Option<Node<M>>
) -> Node<SelectMenuMsg<M>> {
    
    let mut select_menu_classes: Vec<_> = select_menu_model.classes.iter().map(|c| c.to_owned()).collect();
    let select_menu_selectors: Vec<_> = select_menu_model.selectors.iter().collect();
    let mut modal_children = Vec::with_capacity(2);

    if let Some(head) = header {
        modal_children.push(head.map_msg(|m| SelectMenuMsg::Msg(m)));
    }
    modal_children.push(body.map_msg(|m| SelectMenuMsg::Msg(m)));
    if let Some(foot) = footer {
        modal_children.push(foot.map_msg(|m| SelectMenuMsg::Msg(m)));
    }
    
    let mut modal_classes = vec!["SelectMenu-modal".to_owned()];
    if select_menu_model.is_open {
        modal_classes.push("SelectMenu-modal-show".to_owned());
    }
    else {
        modal_classes.push("SelectMenu-modal-hidden".to_owned());
    }

    if select_menu_model.right_aligned {
        select_menu_classes.push("select-menu-container-right-aligned".to_string());
        modal_classes.push("SelectMenu-modal-right-aligned".to_owned());
    }
    

    div(
        &[
            classes(&select_menu_classes),
            selectors(&select_menu_selectors)
        ],
        &[
            button_view(
                &select_menu_model.button_model,
                text_view(
                    &select_menu_model.button_text_model, 
                    &select_menu_model.button_text
                ).map_msg(|_| ButtonMsg::NoAction),
                None
            ).map_msg(|m| SelectMenuMsg::Button(m)),
            div(
                &[
                    classes(&modal_classes)
                ],
                &modal_children
            )
        ]
    )


/*


    .SelectMenu-header {
        display: flex;
        padding: 16px;
        flex: none;
        align-items: center;
        border-bottom: 1px solid var(--color-border-muted);
        }
        @media (min-width: 544px) {
        .SelectMenu-header {
            padding: 7px 7px 7px 16px;
        }
        }

        .SelectMenu-title {
        flex: 1;
        font-size: 14px;
        font-weight: 600;
        }
        @media (min-width: 544px) {
        .SelectMenu-title {
            font-size: inherit;
        }
        }

        .SelectMenu-closeButton {
        padding: 16px;
        margin: -16px;
        line-height: 1;
        color: var(--color-fg-muted);
        background-color: transparent;
        border: 0;
        }
        @media (min-width: 544px) {
        .SelectMenu-closeButton {
            padding: 8px;
            margin: -8px -7px;
        }
        }

        .SelectMenu-filter {
        padding: 16px;
        margin: 0;
        border-bottom: 1px solid var(--color-border-muted);
        }
        @media (min-width: 544px) {
        .SelectMenu-filter {
            padding: 8px;
        }
        }

        .SelectMenu-input {
        display: block;
        width: 100%;
        }
        @media (min-width: 544px) {
        .SelectMenu-input {
            font-size: 14px;
        }
        }

        


        .SelectMenu-list--borderless .SelectMenu-item {
        border-bottom: 0;
        }

        .SelectMenu-icon {
        width: 16px;
        margin-right: 8px;
        flex-shrink: 0;
        }

        .SelectMenu-icon--check {
        visibility: hidden;
        transition: transform 0.12s cubic-bezier(0.5, 0.1, 1, 0.5), visibility 0s 0.12s linear;
        transform: scale(0);
        }

        .SelectMenu-tabs {
        display: flex;
        flex-shrink: 0;
        overflow-x: auto;
        overflow-y: hidden;
        box-shadow: inset 0 -1px 0 var(--color-border-muted);
        -webkit-overflow-scrolling: touch;
        }
        .SelectMenu-tabs::-webkit-scrollbar {
        display: none;
        }
        @media (min-width: 544px) {
        .SelectMenu-tabs {
            padding: 8px 8px 0 8px;
        }
        }

        .SelectMenu-tab {
        flex: 1;
        padding: 8px 16px;
        font-size: 12px;
        font-weight: 500;
        color: var(--color-fg-muted);
        text-align: center;
        background-color: transparent;
        border: 0;
        box-shadow: inset 0 -1px 0 var(--color-border-muted);
        }
        @media (min-width: 544px) {
        .SelectMenu-tab {
            flex: none;
            padding: 4px 16px;
            border: 1px solid transparent;
            border-bottom-width: 0;
            border-top-left-radius: 6px;
            border-top-right-radius: 6px;
        }
        }
        .SelectMenu-tab[aria-selected=true] {
        z-index: 1;
        color: var(--color-fg-default);
        cursor: default;
        background-color: var(--color-canvas-overlay);
        box-shadow: 0 0 0 1px var(--color-border-muted);
        }
        @media (min-width: 544px) {
        .SelectMenu-tab[aria-selected=true] {
            border-color: var(--color-border-muted);
            box-shadow: none;
        }
        }

        .SelectMenu-message {
        padding: 7px 16px;
        text-align: center;
        background-color: var(--color-canvas-overlay);
        border-bottom: 1px solid var(--color-border-muted);
        }

        .SelectMenu-blankslate,
        .SelectMenu-loading {
        padding: 24px 16px;
        text-align: center;
        background-color: var(--color-canvas-overlay);
        }

        .SelectMenu-divider {
        padding: 4px 16px;
        margin: 0;
        font-size: 12px;
        font-weight: 500;
        color: var(--color-fg-muted);
        background-color: var(--color-canvas-subtle);
        border-bottom: 1px solid var(--color-border-muted);
        }
        .SelectMenu-list--borderless .SelectMenu-divider {
        border-top: 1px solid var(--color-border-muted);
        }
        .SelectMenu-list--borderless .SelectMenu-divider:empty {
        padding: 0;
        border-top: 0;
        }

        .SelectMenu-footer {
        z-index: 0;
        padding: 8px 16px;
        font-size: 12px;
        color: var(--color-fg-muted);
        text-align: center;
        border-top: 1px solid var(--color-border-muted);
        }
        @media (min-width: 544px) {
        .SelectMenu-footer {
            padding: 7px 16px;
        }
        }

        .SelectMenu--hasFilter .SelectMenu-modal {
        height: 80%;
        max-height: none;
        margin-top: 0;
        }
        @media (min-width: 544px) {
        .SelectMenu--hasFilter .SelectMenu-modal {
            height: auto;
            max-height: 480px;
            margin-top: 8px;
        }
        }

        .SelectMenu-tab:focus,
        .SelectMenu-item:focus {
        outline: 0;
        }

        .SelectMenu-item:hover {
        text-decoration: none;
        }

        .SelectMenu-item[aria-checked=true] {
        font-weight: 500;
        color: var(--color-fg-default);
        }
        .SelectMenu-item[aria-checked=true] .SelectMenu-icon--check {
        visibility: visible;
        transition: transform 0.12s cubic-bezier(0, 0, 0.2, 1), visibility 0s linear;
        transform: scale(1);
        }

        .SelectMenu-item:disabled,
        .SelectMenu-item[aria-disabled=true] {
        color: var(--color-primer-fg-disabled);
        pointer-events: none;
        }
    */
}