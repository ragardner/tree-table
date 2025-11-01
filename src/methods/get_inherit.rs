use crate::Table;
use crate::str_err;
use std::collections::HashMap;

impl Table {
    pub(crate) fn cr_get_inherit_row(
        &self,
        iid_to_row: &HashMap<String, usize>,
        pos: usize,
        inherit: &str,
    ) -> Result<Option<usize>, String> {
        let inherit_pos: Result<Option<usize>, String> = match inherit {
            "@next" => Ok(Some(pos)),
            "@prev" => Ok(Some(pos.saturating_sub(1))),
            "@none" => Ok(None),
            iid => {
                if let Some(pos) = iid_to_row.get(iid) {
                    Ok(Some(*pos))
                } else {
                    Err(str_err!(format!("Iid: {} does not exist.", iid)))
                }
            }
        };
        inherit_pos
    }

    pub(crate) fn cr_get_inherit_col(
        &self,
        iid_to_col: &HashMap<String, usize>,
        pos: usize,
        inherit: &str,
    ) -> Result<Option<usize>, String> {
        let inherit_pos: Result<Option<usize>, String> = match inherit {
            "@next" => Ok(Some(pos)),
            "@prev" => Ok(Some(pos.saturating_sub(1))),
            "@none" => Ok(None),
            iid => {
                if let Some(pos) = iid_to_col.get(iid) {
                    Ok(Some(*pos))
                } else {
                    Err(str_err!(format!("Iid: {} does not exist.", iid)))
                }
            }
        };
        inherit_pos
    }
}
