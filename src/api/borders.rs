use alloc::{rc::Rc, string::String};
use core::cell::RefCell;

use crate::{Border, BordersArgs, CellBorders, EventData, SetBordersArgs, Table};

impl Table {
    pub fn borders(
        &mut self,
        kwargs: Option<BordersArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: BordersArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let borders = if kwargs.top.is_some()
            || kwargs.bottom.is_some()
            || kwargs.left.is_some()
            || kwargs.right.is_some()
        {
            Some(CellBorders {
                top: Border::try_from_opt(kwargs.top)?,
                right: Border::try_from_opt(kwargs.right)?,
                bottom: Border::try_from_opt(kwargs.bottom)?,
                left: Border::try_from_opt(kwargs.left)?,
            })
        } else {
            None
        };
        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);

        if let Err(e) = (|| -> Result<(), String> {
            if kwargs.expand {
                self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
                self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
            }
            let empty_val = &self.opts.empty_val;

            for row in start_row..=end_row {
                for col in start_col..=end_col {
                    Self::acr_event_data_set_tborders(
                        &mut self.grid.index.cells,
                        row,
                        col,
                        borders.clone(),
                        empty_val,
                        &mut event_data,
                    )?;
                }
            }
            Ok(())
        })() {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
            return Err(e);
        }

        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn borders_i(
        &mut self,
        kwargs: Option<BordersArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: BordersArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let borders = if kwargs.top.is_some()
            || kwargs.bottom.is_some()
            || kwargs.left.is_some()
            || kwargs.right.is_some()
        {
            Some(CellBorders {
                top: Border::try_from_opt(kwargs.top)?,
                right: Border::try_from_opt(kwargs.right)?,
                bottom: Border::try_from_opt(kwargs.bottom)?,
                left: Border::try_from_opt(kwargs.left)?,
            })
        } else {
            None
        };
        let (start_row, end_row) = span.rows_start_end(&self.grid);

        if let Err(e) = (|| -> Result<(), String> {
            if kwargs.expand {
                self.cr_ensure_widget_nrows(span.max_row(&self.grid), &mut event_data, true)?;
            }

            for row in start_row..=end_row {
                Self::acr_event_data_set_iborders(
                    &mut self.grid.index.cells,
                    row,
                    borders.clone(),
                    &mut event_data,
                )?;
            }
            Ok(())
        })() {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
            return Err(e);
        }

        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn borders_h(
        &mut self,
        kwargs: Option<BordersArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs: BordersArgs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data: EventData = self.cr_new_event(kwargs.save_selection);
        let borders = if kwargs.top.is_some()
            || kwargs.bottom.is_some()
            || kwargs.left.is_some()
            || kwargs.right.is_some()
        {
            Some(CellBorders {
                top: Border::try_from_opt(kwargs.top)?,
                right: Border::try_from_opt(kwargs.right)?,
                bottom: Border::try_from_opt(kwargs.bottom)?,
                left: Border::try_from_opt(kwargs.left)?,
            })
        } else {
            None
        };
        let (start_col, end_col) = span.cols_start_end(&self.grid);

        if let Err(e) = (|| -> Result<(), String> {
            if kwargs.expand {
                self.cr_ensure_widget_ncols(span.max_col(&self.grid), &mut event_data, true)?;
            }

            for col in start_col..=end_col {
                Self::acr_event_data_set_hborders(
                    &mut self.grid.header.cells,
                    col,
                    borders.clone(),
                    &mut event_data,
                )?;
            }
            Ok(())
        })() {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
            return Err(e);
        }

        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn set_borders(
        &mut self,
        kwargs: Option<SetBordersArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);

        let top = kwargs.top.map(Border::try_from_opt).transpose()?;
        let right = kwargs.right.map(Border::try_from_opt).transpose()?;
        let bottom = kwargs.bottom.map(Border::try_from_opt).transpose()?;
        let left = kwargs.left.map(Border::try_from_opt).transpose()?;

        let (start_row, end_row) = span.rows_start_end(&self.grid);
        let (start_col, end_col) = span.cols_start_end(&self.grid);
        let empty_val = &self.opts.empty_val;

        if let Err(e) = (|| -> Result<(), String> {
            for row in start_row..=end_row {
                for col in start_col..=end_col {
                    if let Some(b) = &top {
                        Self::acr_event_data_set_tborders_top(
                            &mut self.grid.index.cells,
                            row,
                            col,
                            b.clone(),
                            empty_val,
                            &mut event_data,
                        )?;
                    }
                    if let Some(b) = &right {
                        Self::acr_event_data_set_tborders_right(
                            &mut self.grid.index.cells,
                            row,
                            col,
                            b.clone(),
                            empty_val,
                            &mut event_data,
                        )?;
                    }
                    if let Some(b) = &bottom {
                        Self::acr_event_data_set_tborders_bottom(
                            &mut self.grid.index.cells,
                            row,
                            col,
                            b.clone(),
                            empty_val,
                            &mut event_data,
                        )?;
                    }
                    if let Some(b) = &left {
                        Self::acr_event_data_set_tborders_left(
                            &mut self.grid.index.cells,
                            row,
                            col,
                            b.clone(),
                            empty_val,
                            &mut event_data,
                        )?;
                    }
                }
            }
            Ok(())
        })() {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
            return Err(e);
        }

        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn set_borders_i(
        &mut self,
        kwargs: Option<SetBordersArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);

        let top = kwargs.top.map(Border::try_from_opt).transpose()?;
        let right = kwargs.right.map(Border::try_from_opt).transpose()?;
        let bottom = kwargs.bottom.map(Border::try_from_opt).transpose()?;
        let left = kwargs.left.map(Border::try_from_opt).transpose()?;

        let (start_row, end_row) = span.rows_start_end(&self.grid);

        if let Err(e) = (|| -> Result<(), String> {
            for row in start_row..=end_row {
                if let Some(b) = &top {
                    Self::acr_event_data_set_iborders_top(
                        &mut self.grid.index.cells,
                        row,
                        b.clone(),
                        &mut event_data,
                    )?;
                }
                if let Some(b) = &right {
                    Self::acr_event_data_set_iborders_right(
                        &mut self.grid.index.cells,
                        row,
                        b.clone(),
                        &mut event_data,
                    )?;
                }
                if let Some(b) = &bottom {
                    Self::acr_event_data_set_iborders_bottom(
                        &mut self.grid.index.cells,
                        row,
                        b.clone(),
                        &mut event_data,
                    )?;
                }
                if let Some(b) = &left {
                    Self::acr_event_data_set_iborders_left(
                        &mut self.grid.index.cells,
                        row,
                        b.clone(),
                        &mut event_data,
                    )?;
                }
            }
            Ok(())
        })() {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
            return Err(e);
        }
        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }

    pub fn set_borders_h(
        &mut self,
        kwargs: Option<SetBordersArgs>,
    ) -> Result<Option<Rc<RefCell<EventData>>>, String> {
        let kwargs = kwargs.unwrap_or_default();
        let span = self.cr_key_to_span(kwargs.key)?;
        let mut event_data = self.cr_new_event(kwargs.save_selection);

        let top = kwargs.top.map(Border::try_from_opt).transpose()?;
        let right = kwargs.right.map(Border::try_from_opt).transpose()?;
        let bottom = kwargs.bottom.map(Border::try_from_opt).transpose()?;
        let left = kwargs.left.map(Border::try_from_opt).transpose()?;

        let (start_col, end_col) = span.cols_start_end(&self.grid);

        if let Err(e) = (|| -> Result<(), String> {
            for col in start_col..=end_col {
                if let Some(b) = &top {
                    Self::acr_event_data_set_hborders_top(
                        &mut self.grid.header.cells,
                        col,
                        b.clone(),
                        &mut event_data,
                    )?;
                }
                if let Some(b) = &right {
                    Self::acr_event_data_set_hborders_right(
                        &mut self.grid.header.cells,
                        col,
                        b.clone(),
                        &mut event_data,
                    )?;
                }
                if let Some(b) = &bottom {
                    Self::acr_event_data_set_hborders_bottom(
                        &mut self.grid.header.cells,
                        col,
                        b.clone(),
                        &mut event_data,
                    )?;
                }
                if let Some(b) = &left {
                    Self::acr_event_data_set_hborders_left(
                        &mut self.grid.header.cells,
                        col,
                        b.clone(),
                        &mut event_data,
                    )?;
                }
            }
            Ok(())
        })() {
            self.cr_roll_back(Rc::new(RefCell::new(event_data)));
            return Err(e);
        }
        self.cr_finalize_event_if_any_non_selection_change(
            event_data,
            kwargs.undo,
            kwargs.emit,
            true,
        )
    }
}
