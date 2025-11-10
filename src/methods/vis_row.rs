use crate::EventData;
use crate::IndexCell;
use crate::Table;

impl Table {
    pub(crate) fn acr_set_row_vis<'a, I>(
        cells: &mut Vec<IndexCell>,
        iter: I,
        vis: bool,
        event_data: Option<&mut EventData>,
    ) -> Result<(), String>
    where
        I: IntoIterator<Item = usize>,
    {
        if let Some(event_data) = event_data {
            for pos in iter {
                let cell = Self::acr_icell_mut(cells, pos)?;
                event_data.add_change_icells(pos, cell.record_set_vis(vis));
            }
        } else {
            for pos in iter {
                Self::acr_icell_mut(cells, pos)?.vis = vis;
            }
        }
        Ok(())
    }
}
