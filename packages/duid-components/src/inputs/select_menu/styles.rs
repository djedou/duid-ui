use std::collections::HashSet;


pub(crate) fn default_select_menu_selectors() -> HashSet<String> {
    let mut selectors = HashSet::with_capacity(0);
    
    selectors.insert(".details-reset[open] > summary::before:::fixed inset-0 z-4 block cursor-default content-[\"&\"] bg-color-transparent".to_owned());
    selectors.insert(".details-reset > summary::before:::hidden".to_owned());
    selectors.insert(".select-menu-container:::relative inline-block".to_owned());
    selectors.insert(".select-menu-container-right-aligned:::float-right".to_owned());
    selectors.insert(".SelectMenu-modal:::absolute max-h-max min-w-max z-4 my-1 mx-0 overflow-hidden pointer-events-auto bg-color-gray-50 border border-solid border-slate-400 rounded-md shadow-md shadow-slate-400".to_owned()); //  animation[SelectMenu-modal-animation&0.12s&cubic-bezier(0,&0.1,&0.1,&1)&backwards;@keyframes&SelectMenu-modal-animation&{0%&{opacity:&0;transform:&scale(0.9);}}]
    selectors.insert(".SelectMenu-modal-right-aligned:::right-0".to_owned());
    selectors.insert(".SelectMenu-list:::p-0 mt-0 mr-0 ml-0 mb-0[-1px] bg-color-gray-50".to_owned());
    selectors.insert(".SelectMenu-item:::flex items-center w-full px-4 py-1 overflow-hidden text-left cursor-pointer border-t-0 border-l-0 border-r-0 border-b border-solid border-gray-400".to_owned());

    selectors
}
