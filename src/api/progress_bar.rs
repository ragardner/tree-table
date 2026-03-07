use crate::DelProgressBarNameArgs;
use crate::EventData;
use crate::ProgressBar;
use crate::SetProgressBarNameArgs;
use crate::Table;
use crate::args::AddProgressBarArgs;
use crate::args::DelProgressBarArgs;
use crate::args::args_progress_bar::AddProgressBarPosArgs;
use crate::args::args_progress_bar::SetProgressBarArgs;
use crate::str_err;
use crate::try_hex_to_color3;
use std::cell::RefCell;
use std::rc::Rc;

impl Table {
    pub fn add_progress_bar(
        &mut self,
        kwargs: AddProgressBarArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let progress_bar = Some(ProgressBar {
            bg: try_hex_to_color3(kwargs.bg)?,
            fg: try_hex_to_color3(kwargs.fg)?,
            pc: kwargs.percent,
            remove_on_complete: kwargs.remove_on_complete,
        });
        let row = Self::acr_pos(&self.grid.index.iid_to_row, &kwargs.row)?;
        let col = Self::acr_pos(&self.grid.header.iid_to_col, &kwargs.col)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        Self::acr_event_data_set_tprogressbar(
            &mut self.grid.index.cells,
            row,
            col,
            progress_bar,
            &self.opts.empty_val,
            &mut event_data,
        )?;
        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    pub fn add_progress_bar_i(
        &mut self,
        kwargs: AddProgressBarPosArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let progress_bar = Some(ProgressBar {
            bg: try_hex_to_color3(kwargs.bg)?,
            fg: try_hex_to_color3(kwargs.fg)?,
            pc: kwargs.percent,
            remove_on_complete: kwargs.remove_on_complete,
        });
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        Self::acr_event_data_set_iprogressbar(
            &mut self.grid.index.cells,
            kwargs.pos as usize,
            progress_bar,
            &mut event_data,
        )?;
        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    pub fn add_progress_bar_h(
        &mut self,
        kwargs: AddProgressBarPosArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let progress_bar = Some(ProgressBar {
            bg: try_hex_to_color3(kwargs.bg)?,
            fg: try_hex_to_color3(kwargs.fg)?,
            pc: kwargs.percent,
            remove_on_complete: kwargs.remove_on_complete,
        });
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        Self::acr_event_data_set_hprogressbar(
            &mut self.grid.header.cells,
            kwargs.pos as usize,
            progress_bar,
            &mut event_data,
        )?;
        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    pub fn del_progress_bar(
        &mut self,
        kwargs: DelProgressBarArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let row = Self::acr_pos(&self.grid.index.iid_to_row, &kwargs.row)?;
        let col = Self::acr_pos(&self.grid.header.iid_to_col, &kwargs.col)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        Self::acr_event_data_set_tprogressbar(
            &mut self.grid.index.cells,
            row,
            col,
            None,
            &self.opts.empty_val,
            &mut event_data,
        )?;
        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    pub fn del_progress_bar_i(
        &mut self,
        kwargs: DelProgressBarNameArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let name = kwargs.name;
        let pos = Self::acr_pos(&self.grid.index.iid_to_row, name.as_str())?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        Self::acr_event_data_set_iprogressbar(
            &mut self.grid.index.cells,
            pos,
            None,
            &mut event_data,
        )?;
        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    pub fn del_progress_bar_h(
        &mut self,
        kwargs: DelProgressBarNameArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let name = kwargs.name;
        let pos = Self::acr_pos(&self.grid.header.iid_to_col, name.as_str())?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        Self::acr_event_data_set_hprogressbar(
            &mut self.grid.header.cells,
            pos,
            None,
            &mut event_data,
        )?;
        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    pub fn set_progress_bar(
        &mut self,
        kwargs: SetProgressBarArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let row = Self::acr_pos(&self.grid.index.iid_to_row, &kwargs.row)?;
        let col = Self::acr_pos(&self.grid.header.iid_to_col, &kwargs.col)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);

        // Get current progress bar (if any)
        let current_pb = {
            let cell = Table::acr_icell_mut(&mut self.grid.index.cells, row)?;
            cell.vals
                .get(&col)
                .and_then(|tc| tc.props.as_ref())
                .and_then(|p| p.progress_bar.as_ref())
                .cloned()
        };

        // Determine final state to check for removal rule
        let mut final_pc = current_pb.as_ref().map(|p| p.pc).unwrap_or(0.0);
        let mut final_remove = current_pb
            .as_ref()
            .map(|p| p.remove_on_complete)
            .unwrap_or(false);

        if let Some(p) = kwargs.percent {
            final_pc = p;
        }
        if let Some(r) = kwargs.remove_on_complete {
            final_remove = r;
        }

        // Special rule: if pc >= 1.0 and remove_on_complete is true → delete the bar entirely
        if final_pc >= 1.0 && final_remove {
            Self::acr_event_data_set_tprogressbar(
                &mut self.grid.index.cells,
                row,
                col,
                None,
                &self.opts.empty_val,
                &mut event_data,
            )?;
        } else {
            // Normal granular updates (only the fields that were supplied)
            if let Some(bg_opt) = kwargs.bg {
                let bg = try_hex_to_color3(bg_opt)?;
                Self::acr_event_data_set_tprogressbar_bg(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    bg,
                    &self.opts.empty_val,
                    &mut event_data,
                )?;
            }

            if let Some(fg_opt) = kwargs.fg {
                let fg = try_hex_to_color3(fg_opt)?;
                Self::acr_event_data_set_tprogressbar_fg(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    fg,
                    &self.opts.empty_val,
                    &mut event_data,
                )?;
            }

            if let Some(percent) = kwargs.percent {
                Self::acr_event_data_set_tprogressbar_pc(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    percent,
                    &self.opts.empty_val,
                    &mut event_data,
                )?;
            }

            if let Some(remove_on_complete) = kwargs.remove_on_complete {
                Self::acr_event_data_set_tprogressbar_remove_on_complete(
                    &mut self.grid.index.cells,
                    row,
                    col,
                    remove_on_complete,
                    &self.opts.empty_val,
                    &mut event_data,
                )?;
            }
        }

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    pub fn set_progress_bar_i(
        &mut self,
        kwargs: SetProgressBarNameArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let name = kwargs.name.clone();
        let pos = match Self::acr_pos(&self.grid.index.iid_to_row, &name) {
            Ok(p) => p,
            Err(_) => {
                if kwargs.err_if_not_found {
                    return Err(str_err!("Index progress bar '{}' not found", name));
                } else {
                    return Ok(None);
                }
            }
        };

        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);

        // Get current progress bar state (if any)
        let current_pb = {
            let cell = Table::acr_icell_mut(&mut self.grid.index.cells, pos)?;
            cell.props
                .as_ref()
                .and_then(|p| p.progress_bar.as_ref())
                .cloned()
        };

        // Determine final state to check for removal rule
        let mut final_pc = current_pb.as_ref().map(|p| p.pc).unwrap_or(0.0);
        let mut final_remove = current_pb
            .as_ref()
            .map(|p| p.remove_on_complete)
            .unwrap_or(false);

        if let Some(p) = kwargs.percent {
            final_pc = p;
        }
        if let Some(r) = kwargs.remove_on_complete {
            final_remove = r;
        }

        // Special rule: if pc >= 1.0 and remove_on_complete is true → delete the bar entirely
        if final_pc >= 1.0 && final_remove {
            Self::acr_event_data_set_iprogressbar(
                &mut self.grid.index.cells,
                pos,
                None,
                &mut event_data,
            )?;
        } else {
            // Normal granular updates (only the fields that were supplied)
            if let Some(bg_opt) = kwargs.bg {
                let bg = try_hex_to_color3(bg_opt)?;
                Self::acr_event_data_set_iprogressbar_bg(
                    &mut self.grid.index.cells,
                    pos,
                    bg,
                    &mut event_data,
                )?;
            }

            if let Some(fg_opt) = kwargs.fg {
                let fg = try_hex_to_color3(fg_opt)?;
                Self::acr_event_data_set_iprogressbar_fg(
                    &mut self.grid.index.cells,
                    pos,
                    fg,
                    &mut event_data,
                )?;
            }

            if let Some(percent) = kwargs.percent {
                Self::acr_event_data_set_iprogressbar_pc(
                    &mut self.grid.index.cells,
                    pos,
                    percent,
                    &mut event_data,
                )?;
            }

            if let Some(remove_on_complete) = kwargs.remove_on_complete {
                Self::acr_event_data_set_iprogressbar_remove_on_complete(
                    &mut self.grid.index.cells,
                    pos,
                    remove_on_complete,
                    &mut event_data,
                )?;
            }
        }

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }

    pub fn set_progress_bar_h(
        &mut self,
        kwargs: SetProgressBarNameArgs,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let name = kwargs.name.clone();
        let pos = match Self::acr_pos(&self.grid.header.iid_to_col, &name) {
            Ok(p) => p,
            Err(_) => {
                if kwargs.err_if_not_found {
                    return Err(str_err!("Header progress bar '{}' not found", name));
                } else {
                    return Ok(None);
                }
            }
        };

        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);

        // Get current progress bar state (if any)
        let current_pb = {
            let cell = Table::acr_hcell_mut(&mut self.grid.header.cells, pos)?;
            cell.props
                .as_ref()
                .and_then(|p| p.progress_bar.as_ref())
                .cloned()
        };

        // Determine final state to check for removal rule
        let mut final_pc = current_pb.as_ref().map(|p| p.pc).unwrap_or(0.0);
        let mut final_remove = current_pb
            .as_ref()
            .map(|p| p.remove_on_complete)
            .unwrap_or(false);

        if let Some(p) = kwargs.percent {
            final_pc = p;
        }
        if let Some(r) = kwargs.remove_on_complete {
            final_remove = r;
        }

        // Special rule: if pc >= 1.0 and remove_on_complete is true → delete the bar entirely
        if final_pc >= 1.0 && final_remove {
            Self::acr_event_data_set_hprogressbar(
                &mut self.grid.header.cells,
                pos,
                None,
                &mut event_data,
            )?;
        } else {
            // Normal granular updates (only the fields that were supplied)
            if let Some(bg_opt) = kwargs.bg {
                let bg = try_hex_to_color3(bg_opt)?;
                Self::acr_event_data_set_hprogressbar_bg(
                    &mut self.grid.header.cells,
                    pos,
                    bg,
                    &mut event_data,
                )?;
            }

            if let Some(fg_opt) = kwargs.fg {
                let fg = try_hex_to_color3(fg_opt)?;
                Self::acr_event_data_set_hprogressbar_fg(
                    &mut self.grid.header.cells,
                    pos,
                    fg,
                    &mut event_data,
                )?;
            }

            if let Some(percent) = kwargs.percent {
                Self::acr_event_data_set_hprogressbar_pc(
                    &mut self.grid.header.cells,
                    pos,
                    percent,
                    &mut event_data,
                )?;
            }

            if let Some(remove_on_complete) = kwargs.remove_on_complete {
                Self::acr_event_data_set_hprogressbar_remove_on_complete(
                    &mut self.grid.header.cells,
                    pos,
                    remove_on_complete,
                    &mut event_data,
                )?;
            }
        }

        self.cr_finalize_event(event_data, kwargs.undo, kwargs.emit)
    }
}
