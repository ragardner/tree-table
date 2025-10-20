use crate::EventData;
use crate::Header;
use crate::HeaderCell;
use crate::Table;
use crate::str_err;

impl Table {
    /// Check if a col is visible
    #[inline]
    pub(crate) fn acr_col_is_vis(cells: &Vec<HeaderCell>, col: &usize) -> Result<bool, String> {
        if let Some(cell) = cells.get(*col) {
            Ok(cell.vis)
        } else {
            Err(str_err!(format!("No cell found for col {}", col)))
        }
    }

    /// Check if a col is visible, return Option
    #[inline]
    pub(crate) fn acr_col_is_vis_opt(cells: &Vec<HeaderCell>, col: &usize) -> Option<bool> {
        cells.get(*col).map(|cell| cell.vis)
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
                let cell = Self::acr_hcell_mut(&mut cells, &pos)?;
                event_data.add_change_hcells(&pos, cell.record_set_vis(vis));
            }
        } else {
            for pos in iter {
                Self::acr_hcell_mut(&mut cells, &pos)?.vis = vis;
            }
        }
        Ok(())
    }

    /// Check if a col is visible using a iid
    #[inline]
    pub(crate) fn acr_col_is_vis_iid(header: &Header, iid: &str) -> Result<bool, String> {
        if let Some(col) = header.iid_to_col.get(iid) {
            if let Some(cell) = header.cells.get(*col as usize) {
                Ok(cell.vis)
            } else {
                Err(str_err!(format!("No cell found for col {}", col)))
            }
        } else {
            Err(str_err!(format!("No col found for iid {}", iid)))
        }
    }
}
