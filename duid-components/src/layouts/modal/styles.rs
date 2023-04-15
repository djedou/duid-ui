use std::collections::HashSet;


pub(crate) fn default_modal_selectors() -> HashSet<String> {
    let mut selectors = HashSet::with_capacity(0);
    
    selectors.insert(".details-modal-reset[open] > summary::before:::fixed inset-0 z-4 block cursor-default pointer-events-none content-[\"&\"] bg-color--222-1-91 opacity-50".to_owned());
    selectors.insert(".details-modal-reset > summary::before:::hidden".to_owned());
    selectors.insert(".details-modal-reset > summary::marker:::hidden".to_owned());
    selectors.insert(".details-modal-reset-container:::relative inline-block".to_owned());

    selectors
}
