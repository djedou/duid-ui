use std::collections::HashSet;


pub(crate) fn default_input_selectors() -> HashSet<String> {
    let mut selectors = HashSet::with_capacity(0);
    /*
    selectors.insert(".input-sm:::min-h-0[28px] py-0.5 text-xs[12px] leading-5".to_owned());
    selectors.insert(".input-lg:::text-xs[16px]".to_owned());
    selectors.insert(".input-block:::block w-full".to_owned());
    selectors.insert(".form-group:::my-3.5 mx-0".to_owned());
    selectors.insert(".form-group.flattened dt, .form-group.flattened .form-group-header:::m-0 float-left leading-8".to_owned());
    selectors.insert(".form-group.flattened dd, .form-group.flattened .form-group-body:::leading-8".to_owned());
    selectors.insert(r#".form-group.successed .success,
                .form-group.successed .warning,
                .form-group.successed .error, .form-group.warn .success,
                .form-group.warn .warning,
                .form-group.warn .error, .form-group.errored .success,
                .form-group.errored .warning,
                .form-group.errored .error:::absolute z-10 block max-w-md py-1 px-2 mt-2 mr-0 ml-0 mb-0 text-xs[12px] font-normal border-solid border-0[1px] rounded-md"#.to_owned()
    );
    selectors.insert(r#".form-group.successed .success::after, .form-group.successed .success::before,
                .form-group.successed .warning::after,
                .form-group.successed .warning::before,
                .form-group.successed .error::after,
                .form-group.successed .error::before, .form-group.warn .success::after, .form-group.warn .success::before,
                .form-group.warn .warning::after,
                .form-group.warn .warning::before,
                .form-group.warn .error::after,
                .form-group.warn .error::before, .form-group.errored .success::after, .form-group.errored .success::before,
                .form-group.errored .warning::after,
                .form-group.errored .warning::before,
                .form-group.errored .error::after,
                .form-group.errored .error::before:::absolute bottom-full left-auto[10px] z-10[15] w-0 h-0 pointer-events-none content-[\"&\"] border-solid[solid\"&\"transparent]"#.to_owned()
    );
    selectors.insert(r#".form-group.successed .success::after,
                .form-group.successed .warning::after,
                .form-group.successed .error::after, .form-group.warn .success::after,
                .form-group.warn .warning::after,
                .form-group.warn .error::after, .form-group.errored .success::after,
                .form-group.errored .warning::after,
                .form-group.errored .error::after:::border-4"#.to_owned()
    );
    
    //selectors.insert(".SelectMenu-modal:::absolute z-8 max-h-max min-w-max my-1 mx-0 pointer-events-auto".to_owned()); //  animation[SelectMenu-modal-animation&0.12s&cubic-bezier(0,&0.1,&0.1,&1)&backwards;@keyframes&SelectMenu-modal-animation&{0%&{opacity:&0;transform:&scale(0.9);}}]
    //selectors.insert(".SelectMenu-modal:::absolute max-h-max min-w-max z-4 my-1 mx-0 overflow-hidden  bg-color-gray-50 border border-solid border-slate-400 rounded-md shadow-md shadow-slate-400".to_owned()); //  animation[SelectMenu-modal-animation&0.12s&cubic-bezier(0,&0.1,&0.1,&1)&backwards;@keyframes&SelectMenu-modal-animation&{0%&{opacity:&0;transform:&scale(0.9);}}]
    //selectors.insert(".SelectMenu-modal-right-aligned:::right-0".to_owned());
    //selectors.insert(".SelectMenu-list:::p-0 mt-0 mr-0 ml-0 mb-0[-1px] bg-color-gray-50".to_owned());
    //selectors.insert(".SelectMenu-item:::flex items-center w-full px-4 py-1 overflow-hidden text-left cursor-pointer border-t-0 border-l-0 border-r-0 border-b border-solid border-gray-400".to_owned());
*/
    selectors
}


/*

    .form-group.successed .success::before,
    .form-group.successed .warning::before,
    .form-group.successed .error::before, .form-group.warn .success::before,
    .form-group.warn .warning::before,
    .form-group.warn .error::before, .form-group.errored .success::before,
    .form-group.errored .warning::before,
    .form-group.errored .error::before {
      margin-left: -1px;
      border-width: 6px;
    }
    .form-group.successed .success {
      color: var(--color-fg-default);
      background-color: var(--color-canvas-default);
      background-image: linear-gradient(var(--color-success-subtle), var(--color-success-subtle));
      border-color: var(--color-success-muted);
    }
    .form-group.successed .success::after {
      border-bottom-color: var(--color-success-subtle);
    }
    .form-group.successed .success::before {
      border-bottom-color: var(--color-success-muted);
    }
*/