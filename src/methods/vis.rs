use crate::EventData;
use crate::HeaderCell;
use crate::IndexCell;
use crate::Table;

impl Table {
    pub(crate) fn acr_set_row_vis<'a, I>(
        mut cells: &mut Vec<IndexCell>,
        iter: I,
        vis: bool,
        event_data: Option<&mut EventData>,
    ) -> Result<(), String>
    where
        I: IntoIterator<Item = usize>,
    {
        if let Some(event_data) = event_data {
            for pos in iter {
                Self::acr_event_data_set_ivis(&mut cells, pos, vis, event_data)?;
            }
        } else {
            for pos in iter {
                Self::acr_icell_mut(cells, pos)?.vis = vis;
            }
        }
        Ok(())
    }

    pub(crate) fn acr_set_col_vis<'a, I>(
        mut cells: &mut Vec<HeaderCell>,
        iter: I,
        vis: bool,
        event_data: Option<&mut EventData>,
    ) -> Result<(), String>
    where
        I: IntoIterator<Item = usize>,
    {
        if let Some(event_data) = event_data {
            for pos in iter {
                Self::acr_event_data_set_hvis(&mut cells, pos, vis, event_data)?;
            }
        } else {
            for pos in iter {
                Self::acr_hcell_mut(&mut cells, pos)?.vis = vis;
            }
        }
        Ok(())
    }
}
