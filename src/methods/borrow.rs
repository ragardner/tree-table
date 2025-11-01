use crate::{Options, Selection, Table, TableGrid};
use std::cell::{Ref, RefMut};

impl Table {
    #[inline]
    pub fn cr_mut_borrows(
        &self,
    ) -> (
        RefMut<'_, TableGrid>,
        Ref<'_, Options>,
        RefMut<'_, Selection>,
    ) {
        (
            self.grid.borrow_mut(),
            self.opts.borrow(),
            self.sel.borrow_mut(),
        )
    }
}
