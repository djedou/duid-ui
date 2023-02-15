use std::collections::HashSet;


pub(crate) fn get_tab_nav_selectors() -> HashSet<String> {
    let mut selectors = HashSet::with_capacity(0);

    let _ = selectors.insert(".tabnav:::mt-0 mb-4 border-b border-solid border-gray-200 ".to_owned());
    let _ = selectors.insert(".tabnav-tabs:::flex mb-1[-1px] overflow-auto".to_owned());
    let _ = selectors.insert(".tabnav-tab:::inline-block shrink-0 py-2 px-4 text-xs color-gray-800 no-underline bg-color-transparent border border-solid border-transparent border-b-0 transition-none[color] duration-75[0.2s] ease-in[cubic-bezier(0.3,&0,&0.5,&1)]".to_owned());
    let _ = selectors.insert(".tabnav-tab.selected, .tabnav-tab[aria-selected=true], .tabnav-tab[aria-current]:not([aria-current=false]):::color-gray-800 bg-color-gray-200 border-gray-400 border-b-0 rounded-tl-md rounded-tr-md rounded-br-none rounded-bl-none".to_owned());
    let _ = selectors.insert(".tabnav-tab.selected .octicon, .tabnav-tab[aria-selected=true] .octicon, .tabnav-tab[aria-current]:not([aria-current=false]) .octicon:::color-inherit".to_owned());
    let _ = selectors.insert(".tabnav-tab:hover:::color-gray-800 no-underline duration-75[0.1s]".to_owned());
    let _ = selectors.insert(".tabnav-tab:focus, .tabnav-tab:focus-visible:::rounded-tl-md rounded-tr-md rounded-br-none rounded-bl-none outline-offset-0[-6px]".to_owned());
    let _ = selectors.insert(".tabnav-tab:active:::color-blue-800".to_owned());
    let _ = selectors.insert(".tabnav-tab .octicon:::mr-1.5 color-blue-800".to_owned());
    let _ = selectors.insert(".tabnav-tab .Counter:::ml-1.5 color-inherit".to_owned());
    let _ = selectors.insert(".tabnav-extra:::inline-block pt-2.5 ml-2.5 text-xs color-blue-800".to_owned());
    let _ = selectors.insert(".tabnav-extra > .octicon:::ml-0.5 color-inherit".to_owned());
    let _ = selectors.insert("a.tabnav-extra:hover:::ml-0.5 color-blue-900 no-underline".to_owned());
    let _ = selectors.insert(".tabnav-btn:::ml-2".to_owned());

    selectors
}


/*



 {
  margin-left: 8px;
}



*/