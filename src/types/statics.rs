use std::collections::HashSet;
use std::sync::OnceLock;

pub static EVENT_MODIFIED: &'static str = "TableModified";
pub static EVENT_SELECT: &'static str = "TableSelect";
// pub static EVENT_RENDER: &'static str = "TableRender";
pub static EVENT_CLIPBOARD: &'static str = "TableClipboard";
pub static EVENT_BULK_VALIDATE: &'static str = "BulkValidate";
pub static ROWS: &str = "rows";
pub static COLS: &str = "cols";
pub static CELLS: &str = "cells";
pub static NORMAL: &str = "normal";
pub static HIDDEN: &str = "hidden";
pub static VALID_INHERITS: OnceLock<HashSet<String>> = OnceLock::new();
pub static DEFAULT_INHERIT: &str = "@prev";
pub static DEFAULT_EMIT_EVENT_FOR_API: bool = true;
pub static DEFAULT_UNDO_FOR_API: bool = true;
pub static DEFAULT_SAVE_SELECTION_FOR_API: bool = true;
pub static DEFAULT_EXPAND_FOR_API: bool = true;

#[inline(always)]
pub fn valid_inherits() -> &'static HashSet<String> {
    VALID_INHERITS.get_or_init(|| {
        let mut set = HashSet::new();
        set.insert("@next".to_string());
        set.insert("@prev".to_string());
        set.insert("@none".to_string());
        set
    })
}
#[inline(always)]
pub fn default_inherit() -> String {
    DEFAULT_INHERIT.to_string()
}
#[inline(always)]
pub fn default_emit_event_for_api() -> bool {
    DEFAULT_EMIT_EVENT_FOR_API
}
#[inline(always)]
pub fn default_undo_for_api() -> bool {
    DEFAULT_UNDO_FOR_API
}
#[inline(always)]
pub fn default_save_selection_for_api() -> bool {
    DEFAULT_SAVE_SELECTION_FOR_API
}
#[inline(always)]
pub fn default_expand_for_api() -> bool {
    DEFAULT_EXPAND_FOR_API
}
#[inline(always)]
pub fn get_true() -> bool {
    true
}
