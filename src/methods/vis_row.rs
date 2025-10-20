use crate::EventData;
use crate::Index;
use crate::IndexCell;
use crate::Table;
use crate::str_err;

impl Table {
    /// Check if a row is visible
    #[inline]
    pub(crate) fn acr_row_is_vis(cells: &Vec<IndexCell>, row: &usize) -> Result<bool, String> {
        if let Some(cell) = cells.get(*row) {
            Ok(cell.vis)
        } else {
            Err(str_err!(format!("No cell found for row {}", row)))
        }
    }

    /// Check if a row is visible, return Option
    #[inline]
    pub(crate) fn acr_row_is_vis_opt(cells: &Vec<IndexCell>, row: &usize) -> Option<bool> {
        cells.get(*row).map(|cell| cell.vis)
    }

    /// Check if a row is visible using an iid
    #[inline]
    pub(crate) fn acr_row_is_vis_iid(index: &Index, iid: &str) -> Result<bool, String> {
        if let Some(row) = index.iid_to_row.get(iid) {
            if let Some(cell) = index.cells.get(*row) {
                Ok(cell.vis)
            } else {
                Err(str_err!(format!("No cell found for row {}", row)))
            }
        } else {
            Err(str_err!(format!("No row found for iid {}", iid)))
        }
    }

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
                let cell = Self::acr_icell_mut(&mut cells, &pos)?;
                event_data.add_change_icells(&pos, cell.record_set_vis(vis));
            }
        } else {
            for pos in iter {
                Self::acr_icell_mut(&mut cells, &pos)?.vis = vis;
            }
        }
        Ok(())
    }

    pub(crate) fn cr_set_row_vis_result<'a, I>(
        mut cells: &mut Vec<IndexCell>,
        iter: I,
        vis: bool,
        event_data: Option<&mut EventData>,
    ) -> Result<(), String>
    where
        I: IntoIterator<Item = Result<usize, String>>,
    {
        let iter = iter.into_iter();
        if let Some(event_data) = event_data {
            for res in iter {
                let pos = res?;
                let cell = Self::acr_icell_mut(&mut cells, &pos)?;
                event_data.add_change_icells(&pos, cell.record_set_vis(vis));
            }
        } else {
            for res in iter {
                Self::acr_icell_mut(&mut cells, &res?)?.vis = vis;
            }
        }
        Ok(())
    }
}
