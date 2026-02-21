use crate::BoundedDeque;
use crate::EventData;
use crate::Header;
use crate::Index;
use crate::utils::utils_fns::pull_n;
use crate::utils::utils_fns::push_n;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct TableGrid {
    pub index: Index,
    pub header: Header,
    pub merged: HashMap<(usize, usize), (usize, usize)>,
    pub frozen_rows: Vec<usize>,
    pub frozen_cols: Vec<usize>,
    pub named_cells: HashMap<String, (usize, usize)>,
    pub disp_rows: Vec<usize>,
    pub disp_cols: Vec<usize>,
    pub canvas_rows: Vec<u64>,
    pub canvas_cols: Vec<u64>,
    pub undo_stack: BoundedDeque<Rc<RefCell<EventData>>>,
    pub redo_stack: BoundedDeque<Rc<RefCell<EventData>>>,
}

impl TableGrid {
    pub fn new(max_undos: usize) -> Self {
        TableGrid {
            index: Index::new(),
            header: Header::new(),
            merged: HashMap::new(),
            frozen_rows: Vec::new(),
            frozen_cols: Vec::new(),
            named_cells: HashMap::new(),
            disp_rows: Vec::new(),
            disp_cols: Vec::new(),
            canvas_rows: Vec::new(),
            canvas_cols: Vec::new(),
            undo_stack: BoundedDeque::new(max_undos),
            redo_stack: BoundedDeque::new(max_undos),
        }
    }

    /// Resets all but index, header and undo/redo stacks.
    pub fn reset(&mut self) {
        self.merged.clear();
        self.frozen_rows.clear();
        self.frozen_cols.clear();
        self.named_cells.clear();
        self.disp_rows.clear();
        self.disp_cols.clear();
        self.canvas_rows.clear();
        self.canvas_cols.clear();
    }

    pub fn push_rows(&mut self, sorted_seq: &Vec<usize>) {
        self.named_cells = self
            .named_cells
            .drain()
            .map(|(k, v)| (k, (push_n(v.0, sorted_seq), v.1)))
            .collect();
        self.frozen_rows
            .iter_mut()
            .for_each(|k| *k = push_n(*k, sorted_seq));
    }

    pub fn push_cols(&mut self, sorted_seq: &Vec<usize>) {
        self.named_cells = self
            .named_cells
            .drain()
            .map(|(k, v)| (k, (v.0, push_n(v.1, sorted_seq))))
            .collect();
        self.frozen_cols
            .iter_mut()
            .for_each(|k| *k = push_n(*k, sorted_seq));
    }

    pub fn pull_rows(&mut self, sorted_seq: &Vec<usize>) {
        self.named_cells = self
            .named_cells
            .drain()
            .map(|(k, v)| (k, (pull_n(v.0, sorted_seq), v.1)))
            .collect();
        self.frozen_rows
            .iter_mut()
            .for_each(|k| *k = pull_n(*k, sorted_seq));
    }

    pub fn pull_cols(&mut self, sorted_seq: &Vec<usize>) {
        self.named_cells = self
            .named_cells
            .drain()
            .map(|(k, v)| (k, (v.0, pull_n(v.1, sorted_seq))))
            .collect();
        self.frozen_cols
            .iter_mut()
            .for_each(|k| *k = pull_n(*k, sorted_seq));
    }

    pub fn move_rows(
        &mut self,
        full_old_to_new: &Vec<usize>,
        full_new_to_old: &HashMap<usize, usize>,
    ) -> Result<(), String> {
        self.index.move_rows(full_new_to_old)?;
        self.named_cells = self
            .named_cells
            .drain()
            .map(|(k, v)| (k, (full_old_to_new[v.0 as usize], v.1)))
            .collect();
        self.frozen_rows
            .iter_mut()
            .for_each(|k| *k = full_old_to_new[*k as usize]);
        Ok(())
    }

    pub fn move_cols(
        &mut self,
        full_old_to_new: &Vec<usize>,
        full_new_to_old: &HashMap<usize, usize>,
    ) -> Result<(), String> {
        self.index.move_cols(full_old_to_new);
        self.header.move_cols(full_new_to_old)?;
        self.named_cells = self
            .named_cells
            .drain()
            .map(|(k, v)| (k, (v.0, full_old_to_new[v.1 as usize])))
            .collect();
        self.frozen_cols
            .iter_mut()
            .for_each(|k| *k = full_old_to_new[*k as usize]);
        Ok(())
    }
}
