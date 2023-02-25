use std::collections::HashSet;


pub(crate) fn default_select_menu_selectors() -> HashSet<String> {
    let mut selectors = HashSet::with_capacity(0);
/*
    selectors.push(".details-overlay[open] > summary::before:::fixed inset-0 z-98 block cursor-default content-[\"&\"] bg-color-transparent".to_owned());
    selectors.push(".details-reset > summary:::list-none w-fit duration-75 ease-linear[cubic-bezier(0.33,&1,&0.68,&1)] transition-none[color,&background-color,&box-shadow,&border-color]".to_owned());
    selectors.push(".details-reset > summary:focus:::outline outline-2 outline-slate-400 outline-offset-2[-2px] shadow-none".to_owned());
    selectors.push(".details-reset > summary:focus:not(:focus-visible):::outline outline-1 outline-transparent".to_owned());
    selectors.push(".details-reset > summary:focus-visible:::outline outline-2 outline-slate-400 outline-offset-2[-2px]".to_owned());
    selectors.push(".details-reset > summary.btn-filled:focus:::outline outline-2 outline-slate-400 outline-offset-2[-2px] shadow-none[inset&0&0&0&3px&shadow-gray-400]".to_owned());
    selectors.push(".details-reset > summary.btn-filled:focus:not(:focus-visible):::outline outline-1 outline-transparent shadow-none".to_owned());
    selectors.push(".details-reset > summary.btn-filled:focus-visible:::outline outline-2 outline-slate-400 outline-offset-2[-2px] shadow-none[inset&0&0&0&3px&shadow-gray-400]".to_owned());
    selectors.push(".details-reset > summary::before:::hidden".to_owned());
    selectors.push(".details-reset > summary::-webkit-details-marker:::hidden".to_owned());
    selectors.push(".details-overlay > summary:::duration-75 ease-linear[cubic-bezier(0.33,&1,&0.68,&1)] transition-none[color,&background-color,&box-shadow,&border-color]".to_owned());
    selectors.push(".details-overlay > summary:focus:::outline outline-2 outline-slate-400 outline-offset-2[-2px] shadow-none".to_owned());
    selectors.push(".details-overlay > summary:focus:not(:focus-visible):::outline outline-1 outline-transparent".to_owned());
    selectors.push(".details-overlay > summary:focus-visible:::outline outline-2 outline-slate-400 outline-offset-2[-2px] shadow-none".to_owned());
    selectors.push(".details-overlay > summary.btn-filled:focus:::outline outline-2 outline-slate-400 outline-offset-2[-2px] shadow-none[inset&0&0&0&3px&shadow-gray-400]".to_owned());
    selectors.push(".details-overlay > summary.btn-filled:focus:not(:focus-visible):::outline outline-1 outline-transparent shadow-none".to_owned());
    selectors.push(".details-overlay > summary.btn-filled:focus-visible:::outline outline-2 outline-slate-400 outline-offset-2[-2px] shadow-none[inset&0&0&0&3px&shadow-gray-400]".to_owned());
    
    
    selectors.insert(".SelectMenu:::w-fit relative inset-0 z-98 flex p-1 pointer-events-none flex-col".to_owned());
    selectors.insert(".SelectMenu::before:::absolute inset-0 z-98 flex p-4 pointer-events-none content-[\"&\"]".to_owned());
    */
    selectors.insert(".select-menu-container:::relative inline-block".to_owned());
    selectors.insert(".SelectMenu-modal:::inset-0 z-98 max-h-0[66%] my-1 mx-0 overflow-hidden pointer-events-auto bg-color-gray-50 border border-solid border-slate-400 rounded-md shadow-md shadow-slate-400".to_owned()); //  animation[SelectMenu-modal-animation&0.12s&cubic-bezier(0,&0.1,&0.1,&1)&backwards;@keyframes&SelectMenu-modal-animation&{0%&{opacity:&0;transform:&scale(0.9);}}]
    selectors.insert(".SelectMenu-modal-hidden:::hidden".to_owned());
    selectors.insert(".SelectMenu-modal-show:::absolute block".to_owned()); //  w-fit
    selectors.insert(".SelectMenu-list:::flex flex-col p-0 mt-0 mr-0 ml-0 mb-0[-1px] flex-auto overflow-x-hidden overflow-y-auto bg-color-gray-50".to_owned());
    selectors.insert(".SelectMenu-item:::flex items-center w-full px-4 py-1 overflow-hidden text-left cursor-pointer border-t-0 border-l-0 border-r-0 border-b border-solid border-gray-400".to_owned());

    selectors
}
