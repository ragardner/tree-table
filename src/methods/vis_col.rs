use crate::EventData;
use crate::HeaderCell;
use crate::Table;

impl Table {
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
                let cell = Self::acr_hcell_mut(&mut cells, pos)?;
                event_data.add_change_hcells(pos, cell.record_set_vis(vis));
            }
        } else {
            for pos in iter {
                Self::acr_hcell_mut(&mut cells, pos)?.vis = vis;
            }
        }
        Ok(())
    }
}
