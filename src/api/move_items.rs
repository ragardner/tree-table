use crate::EventData;
use crate::Table;
use crate::TreeMove;
use crate::args::MoveItemsArgs;
use crate::str_err;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cell::RefCell;
use hashbrown::HashMap;
use hashbrown::HashSet;

impl Table {
    pub fn move_items(
        &mut self,
        kwargs: MoveItemsArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let num_rows = Self::acr_total_rows(&self.grid.index.cells);
        if num_rows == 0 {
            return Ok(None);
        }
        let Some(items) = kwargs.items else {
            return Ok(None);
        };

        // Item ids
        // Parent
        let new_par: String = match kwargs.par {
            Some(par) => par,
            None => "".to_string(),
        };
        if !new_par.is_empty() && !Self::acr_tree_exists(&self.grid.index.iid_to_row, &new_par) {
            return Err(str_err!("Parent does not exist: {}", new_par));
        };

        // Check items
        let mut hash_items: HashSet<Rc<str>> = HashSet::new();
        for iid in items.into_iter() {
            if !Self::acr_tree_exists(&self.grid.index.iid_to_row, &iid) {
                return Err(str_err!("iid does not exist: {}", &iid));
            }
            if Self::acr_tree_move_pid_causes_recursive_loop(
                &self.grid.index.cells,
                &self.grid.index.iid_to_row,
                &iid,
                &new_par,
            )? {
                return Err(str_err!(
                    "Moving '{}' to '{}' would cause a recursive loop",
                    &iid,
                    new_par
                ));
            }
            hash_items.insert(Rc::from(iid));
        }

        // Create a new event
        let mut event_data = self.cr_new_event(kwargs.save_selection);

        // Set up TreeMove object
        let move_op: TreeMove = TreeMove {
            iids: hash_items,
            new_par: Rc::from(new_par),
            pos: kwargs.pos,
        };

        // Perform move operation
        match self.cr_move_rows(
            &mut event_data,
            HashSet::new(),
            Vec::new(),
            HashMap::new(),
            Some(move_op),
            true,
        ) {
            Err(e) => {
                self.cr_roll_back(Rc::new(RefCell::new(event_data)));
                return Err(e);
            }
            _ => {}
        };

        // Select moved rows if true
        if kwargs.select {
            Self::acr_deselect_all(&mut self.sel, None);
            let mut moved: Vec<usize> = self.extract_disp_moved_rows(&event_data);
            if !moved.is_empty() {
                moved.sort_unstable();
                self.cr_select_rows_from(moved);
            }
        }

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }
}
