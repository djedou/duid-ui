use std::collections::HashSet;



pub(crate) fn get_button_default_selectors() -> HashSet<String> {
    let mut selectors = HashSet::with_capacity(0);

    let _ = selectors.insert(".btn:::font-medium leading-5 whitespace-nowrap align-middle cursor-pointer select-none appearance-none".to_owned());
    let _ = selectors.insert(".btn:disabled, .btn.disabled, .btn[aria-disabled=true]:::cursor-default".to_owned());
    let _ = selectors.insert(".btn i:::not-italic font-medium opacity-75".to_owned());
    let _ = selectors.insert(".btn .octicon:::mr-1.5 color-stone-400".to_owned());
    let _ = selectors.insert(".btn .octicon:only-child:::mr-0".to_owned());

    // ButtonKind::Default
    let _ = selectors.insert(".btn-default:::color-btn-text bg-color-btn border-color-btn shadow-btn-default shadow-color-btn-default-inset transition".to_owned());
    let _ = selectors.insert(".btn-default:hover, .btn-default.hover, [open] > .btn-default:::bg-color-btn-hover border-color-btn-hover duration-1000".to_owned());
    let _ = selectors.insert(".btn-default:active:::bg-color-btn-active border-color-btn-active transition-none".to_owned());
    let _ = selectors.insert(".btn-default.selected, .btn-default[aria-selected=true]:::bg-color-btn-selected shadow-btn-active shadow-color-btn-active".to_owned());
    let _ = selectors.insert(".btn-default:disabled, .btn-default.disabled, .btn-default[aria-disabled=true]:::bg-color-btn color-gray-300 border-color-btn".to_owned());
    let _ = selectors.insert(".btn-default:disabled .octicon, .btn-default.disabled .octicon, .btn-default[aria-disabled=true] .octicon:::color-gray-300".to_owned());
    let _ = selectors.insert(".btn-default .Counter:::color-inherit align-top ml-0.5 bg-color-btn-counter".to_owned());
    let _ = selectors.insert(".btn .dropdown-caret:::ml-1 opacity-80".to_owned());

    // ButtonKind::Filled
    let _ = selectors.insert(".btn-filled:::shadow-btn-filled shadow-color-btn-filled shadow-color-btn-filled-inset".to_owned());
    let _ = selectors.insert(".btn-filled:focus, .btn-filled:focus-visible:::outline-2 outline-offset-2[-2px] outline shadow-btn-focus shadow-color-btn-focus".to_owned());
    let _ = selectors.insert(".btn-filled:focus:not(:focus-visible):::outline-none shadow-none".to_owned());
    let _ = selectors.insert(".btn-filled:active, .btn-filled.selected, .btn-filled[aria-selected=true]:::shadow-btn-focus shadow-color-btn-focus".to_owned());
    let _ = selectors.insert(".btn-filled:disabled, .btn-filled.disabled, .btn-filled[aria-disabled=true]:::color-white".to_owned());
    let _ = selectors.insert(".btn-filled:disabled .octicon, .btn-filled.disabled .octicon, .btn-filled[aria-disabled=true] .octicon:::color-btn-filled-disabled-text".to_owned());
    let _ = selectors.insert(".btn-filled .Counter:::color-inherit bg-color-btn-filled-counter".to_owned());
    let _ = selectors.insert(".btn-filled .octicon:::color-btn-filled-icon".to_owned());

    // ButtonColor::Filled Color Default
    let _ = selectors.insert(".btn-filled-colors:::color-white bg-color-btn-filled border-color-btn-filled".to_owned());
    let _ = selectors.insert(".btn-filled-colors:hover, .btn-filled-colors.hover, [open] > .btn-filled-colors:::bg-color-btn-filled-hover border-color-btn-filled-hover".to_owned());
    let _ = selectors.insert(".btn-filled-colors:focus, .btn-filled-colors:focus-visible:::outline-green-400".to_owned());
    let _ = selectors.insert(".btn-filled-colors:active, .btn-filled-colors.selected, .btn-filled-colors[aria-selected=true]:::bg-color-btn-filled-selected".to_owned());
    let _ = selectors.insert(".btn-filled-colors:disabled, .btn-filled-colors.disabled, .btn-filled-colors[aria-disabled=true]:::bg-color-btn-filled-disabled border-color-btn-filled-disabled".to_owned());

    // ButtonColor::Filled Color Danger
    let _ = selectors.insert(".btn-filled-danger-colors:::color-white bg-color-red-500 border-red-600".to_owned());
    let _ = selectors.insert(".btn-filled-danger-colors:hover, .btn-filled-danger-colors.hover, [open] > .btn-filled-danger-colors:::bg-color-red-600".to_owned());
    let _ = selectors.insert(".btn-filled-danger-colors:focus, .btn-filled-danger-colors:focus-visible:::outline-green-400".to_owned());
    let _ = selectors.insert(".btn-filled-danger-colors:active, .btn-filled-danger-colors.selected, .btn-filled-danger-colors[aria-selected=true]:::bg-color-red-600".to_owned());
    let _ = selectors.insert(".btn-filled-danger-colors:disabled, .btn-filled-danger-colors.disabled, .btn-filled-danger-colors[aria-disabled=true]:::bg-color-red-200 border-red-300".to_owned());

    // ButtonKind::Outline
    let _ = selectors.insert(".btn-outline:hover, [open] > .btn-outline:::color-white border-color-btn-outline-hover shadow-btn-outline-hover shadow-color-btn-outline-hover shadow-color-btn-outline-hover-inset".to_owned());
    let _ = selectors.insert(".btn-outline:hover .Counter, [open] > .btn-outline .Counter:::bg-color-btn-outline-hover-counter".to_owned());
    let _ = selectors.insert(".btn-outline:hover .octicon, [open] > .btn-outline .octicon:::color-inherit".to_owned());
    let _ = selectors.insert(".btn-outline:active, .btn-outline.selected, .btn-outline[aria-selected=true]:::color-white border-color-btn-outline-selected shadow-btn-outline-selected shadow-color-btn-outline-selected".to_owned());
    let _ = selectors.insert(".btn-outline:active:focus, .btn-outline.selected:focus, .btn-outline[aria-selected=true]:focus:::outline-2 outline-offset-2[-2px] outline shadow-btn-focus shadow-color-btn-focus".to_owned());
    let _ = selectors.insert(".btn-outline:active:focus:not(:focus-visible), .btn-outline.selected:focus:not(:focus-visible), .btn-outline[aria-selected=true]:focus:not(:focus-visible):::shadow-none outline-none".to_owned());
    let _ = selectors.insert(".btn-outline:active:focus-visible, .btn-outline.selected:focus-visible, .btn-outline[aria-selected=true]:focus-visible:::outline-2 outline-offset-2[-2px] outline shadow-btn-focus shadow-color-btn-focus".to_owned());
    let _ = selectors.insert(".btn-outline:disabled, .btn-outline.disabled, .btn-outline[aria-disabled=true]:::color-btn-outline-disabled-text border-color-btn shadow-none".to_owned());
    let _ = selectors.insert(".btn-outline:disabled .Counter, .btn-outline.disabled .Counter, .btn-outline[aria-disabled=true] .Counter:::bg-color-btn-outline-disabled-counter".to_owned());
    let _ = selectors.insert(".btn-outline .Counter:::color-inherit bg-color-btn-outline-counter".to_owned());

    // ButtonColor::Outline Color Default
    let _ = selectors.insert(".btn-outline-colors:::color-btn-outline-text border-blue-600".to_owned());
    let _ = selectors.insert(".btn-outline-colors:hover, [open] > .btn-outline:::bg-color-btn-outline-hover border-color-btn-outline-hover".to_owned());
    let _ = selectors.insert(".btn-outline-colors:active, .btn-outline-colors.selected, .btn-outline-colors[aria-selected=true]:::bg-color-btn-outline-selected".to_owned());
    let _ = selectors.insert(".btn-outline-colors:active:focus, .btn-outline-colors.selected:focus, .btn-outline-colors[aria-selected=true]:focus:::outline-blue-400".to_owned());
    let _ = selectors.insert(".btn-outline-colors:active:focus-visible, .btn-outline-colors.selected:focus-visible, .btn-outline-colors[aria-selected=true]:focus-visible:::outline-blue-400".to_owned());
    let _ = selectors.insert(".btn-outline-colors:disabled, .btn-outline-colors.disabled, .btn-outline-colors[aria-disabled=true]:::bg-color-btn-outline-disabled".to_owned());

    // ButtonColor::Outline Color Danger
    let _ = selectors.insert(".btn-outline-colors-danger:::color-red-500 border-red-500".to_owned());
    let _ = selectors.insert(".btn-outline-colors-danger:hover, [open] > .btn-outline-colors-danger:::bg-color-red-500 border-red-500".to_owned());
    let _ = selectors.insert(".btn-outline-colors-danger:active, .btn-outline-colors-danger.selected, .btn-outline-colors-danger[aria-selected=true]:::bg-color-red-600".to_owned());
    let _ = selectors.insert(".btn-outline-colors-danger:active:focus, .btn-outline-colors-danger.selected:focus, .btn-outline-colors-danger[aria-selected=true]:focus:::outline-red-400".to_owned());
    let _ = selectors.insert(".btn-outline-colors-danger:active:focus-visible, .btn-outline-colors-danger.selected:focus-visible, .btn-outline-colors-danger[aria-selected=true]:focus-visible:::outline-red-400".to_owned());
    let _ = selectors.insert(".btn-outline-colors-danger:disabled, .btn-outline-colors-danger.disabled, .btn-outline-colors-danger[aria-disabled=true]:::bg-color-red-200".to_owned());

    // Button Size sm
    let _ = selectors.insert(".btn-sm:::py-1 px-3 text-xs".to_owned());
    let _ = selectors.insert(".btn-sm .octicon:::align-text-top".to_owned());

    // Button Size md
    let _ = selectors.insert(".btn-md:::py-1.5 px-4 text-sm".to_owned());
    let _ = selectors.insert(".btn-md .octicon:::align-text-bottom".to_owned());

    // Button Size lg
    let _ = selectors.insert(".btn-lg:::py-2 px-5 text-base".to_owned());
    let _ = selectors.insert(".btn-lg .octicon:::align-text-bottom".to_owned());

    // Button Icon Only
    let _ = selectors.insert(".btn-octicon:::inline-block p-1 ml-1 leading-none color-white align-middle bg-color-transparent border-0 shadow-none".to_owned());
    let _ = selectors.insert(".btn-octicon:hover:::color-slate-300".to_owned());
    let _ = selectors.insert(".btn-octicon:focus, .btn-octicon:focus-visible:::rounded-md".to_owned());
    let _ = selectors.insert(".btn-octicon.disabled, .btn-octicon[aria-disabled=true]:::color-slate-100 cursor-default".to_owned());
    let _ = selectors.insert(".btn-octicon.disabled:hover, .btn-octicon[aria-disabled=true]:hover:::color-slate-100".to_owned());

    // Button Icon Close
    let _ = selectors.insert(".close-button:::p-0 color-slate-50 bg-color-transparent border-0".to_owned());
    let _ = selectors.insert(".close-button:hover:::color-slate-300".to_owned());
    let _ = selectors.insert(".close-button:active:::outline-2 outline-offset-2[-2px] outline shadow-none".to_owned());

    // Button Icon HiddenText
    let _ = selectors.insert(".hidden-text-expander:::block".to_owned());
    let _ = selectors.insert(".hidden-text-expander.inline:::relative top-px[-1px] inline-block ml-1 leading-none[0]".to_owned());
    let _ = selectors.insert(".hidden-text-expander a, .ellipsis-expander:::inline-block h-3 pt-0 pb-1 px-1 text-xs font-semibold leading-none[6px] color-slate-50 bg-color-gray-500 no-underline[none] align-middle border-0 rounded-none[1px]".to_owned());
    let _ = selectors.insert(".hidden-text-expander a:hover, .ellipsis-expander:hover:::bg-color-gray-500 no-underline[none]".to_owned());
    let _ = selectors.insert(".hidden-text-expander a:active, .ellipsis-expander:active:::bg-color-gray-500 color-slate-100".to_owned());

    // Button Icon Count
    let _ = selectors.insert(".btn-with-count:::float-left rounded-tr-none rounded-br-none".to_owned());
    let _ = selectors.insert(".btn-with-count:focus:::z-1".to_owned());
    let _ = selectors.insert(".social-count:::relative float-left py-1 px-3 text-xs font-semibold leading-5 align-middle color-slate-50 bg-color-gray-500 border border-color-btn border-l-0 rounded-tr-md rounded-br-md shadow-btn-filled shadow-color-btn-filled shadow-color-btn-filled-inset".to_owned());
    let _ = selectors.insert(".social-count:hover, .social-count:active:::no-underline[none]".to_owned());
    let _ = selectors.insert(".social-count:hover:::color-slate-100 cursor-pointer".to_owned());

    // Button Variaton Normal
    let _ = selectors.insert(".btn-normal:::inline-block border border-solid rounded-md".to_owned());
    let _ = selectors.insert(".btn-normal:hover:::no-underline".to_owned());
    let _ = selectors.insert(".btn-normal:hover:::no-underline".to_owned());

    // Button Variaton Block
    let _ = selectors.insert(".btn-block:::block w-full text-center border border-solid rounded-md".to_owned());
    let _ = selectors.insert(".btn-block:hover:::no-underline".to_owned());
    let _ = selectors.insert(".btn-block:hover:::no-underline".to_owned());

    // Button Variaton Link
    let _ = selectors.insert(".btn-link:::inline-block rounded-md p-0 border-0 no-underline bg-color-transparent".to_owned());
    let _ = selectors.insert(".btn-link:hover:::underline".to_owned());
    let _ = selectors.insert(".btn-link:not(.dropdown-item):focus, .btn-link:not(.dropdown-item):focus-visible:::rounded-md outline-2 outline-offset-2[0] outline".to_owned());

    selectors
}

/*


.BtnGroup {
  display: inline-block;
  vertical-align: middle;
}
.BtnGroup::before {
  display: table;
  content: "";
}
.BtnGroup::after {
  display: table;
  clear: both;
  content: "";
}
.BtnGroup + .BtnGroup,
.BtnGroup + .btn {
  margin-left: 4px;
}

.BtnGroup-item {
  position: relative;
  float: left;
  border-right-width: 0;
  border-radius: 0;
}
.BtnGroup-item:first-child {
  border-top-left-radius: 6px;
  border-bottom-left-radius: 6px;
}
.BtnGroup-item:last-child {
  border-right-width: 1px;
  border-top-right-radius: 6px;
  border-bottom-right-radius: 6px;
}
.BtnGroup-item.selected, .BtnGroup-item[aria-selected=true], .BtnGroup-item:focus, .BtnGroup-item:active, .BtnGroup-item:hover {
  border-right-width: 1px;
}
.BtnGroup-item.selected + .BtnGroup-item,
.BtnGroup-item.selected + .BtnGroup-parent .BtnGroup-item, .BtnGroup-item[aria-selected=true] + .BtnGroup-item,
.BtnGroup-item[aria-selected=true] + .BtnGroup-parent .BtnGroup-item, .BtnGroup-item:focus + .BtnGroup-item,
.BtnGroup-item:focus + .BtnGroup-parent .BtnGroup-item, .BtnGroup-item:active + .BtnGroup-item,
.BtnGroup-item:active + .BtnGroup-parent .BtnGroup-item, .BtnGroup-item:hover + .BtnGroup-item,
.BtnGroup-item:hover + .BtnGroup-parent .BtnGroup-item {
  border-left-width: 0;
}

.BtnGroup-parent {
  float: left;
}
.BtnGroup-parent:first-child .BtnGroup-item {
  border-top-left-radius: 6px;
  border-bottom-left-radius: 6px;
}
.BtnGroup-parent:last-child .BtnGroup-item {
  border-right-width: 1px;
  border-top-right-radius: 6px;
  border-bottom-right-radius: 6px;
}
.BtnGroup-parent .BtnGroup-item {
  border-right-width: 0;
  border-radius: 0;
}
.BtnGroup-parent.selected .BtnGroup-item, .BtnGroup-parent[aria-selected=true] .BtnGroup-item, .BtnGroup-parent:focus .BtnGroup-item, .BtnGroup-parent:active .BtnGroup-item, .BtnGroup-parent:hover .BtnGroup-item {
  border-right-width: 1px;
}
.BtnGroup-parent.selected + .BtnGroup-item,
.BtnGroup-parent.selected + .BtnGroup-parent .BtnGroup-item, .BtnGroup-parent[aria-selected=true] + .BtnGroup-item,
.BtnGroup-parent[aria-selected=true] + .BtnGroup-parent .BtnGroup-item, .BtnGroup-parent:focus + .BtnGroup-item,
.BtnGroup-parent:focus + .BtnGroup-parent .BtnGroup-item, .BtnGroup-parent:active + .BtnGroup-item,
.BtnGroup-parent:active + .BtnGroup-parent .BtnGroup-item, .BtnGroup-parent:hover + .BtnGroup-item,
.BtnGroup-parent:hover + .BtnGroup-parent .BtnGroup-item {
  border-left-width: 0;
}

.BtnGroup-item:focus, .BtnGroup-item:active,
.BtnGroup-parent:focus,
.BtnGroup-parent:active {
  z-index: 1;
}

*/
