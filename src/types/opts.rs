use crate::Color3;
use crate::Table;
use crate::Val;
use crate::utils::utils_fns::generate_unique_id;

macro_rules! define_options {
    ($($field:ident: $ty:ty, $default_str:literal => $default:expr),* $(,)?) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Debug, Clone, PartialEq)]
        pub struct Options {
            $(
                pub $field: $ty
            ),*
        }
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Debug, Clone, PartialEq)]
        pub struct PartialOptions {
            $(
                pub $field: Option<$ty>
            ),*
        }
        $(
            pub fn $field() -> $ty { $default }
        )*

        impl Default for Options {
            fn default() -> Self {
                Self {
                    $(
                        $field: $default,
                    )*
                }
            }
        }

        impl Default for PartialOptions {
            fn default() -> Self {
                Self {
                    $(
                        $field: None,
                    )*
                }
            }
        }

        impl PartialOptions {
            pub fn with_defaults() -> Self {
                Self {
                    $(
                        $field: Some($field()),
                    )*
                }
            }
        }

        macro_rules! merge_self_opts_fields {
            ($self:expr, $partial:ident) => {
                $(
                    if let Some(val) = $partial.$field {
                        $self.opts.$field = val;
                    }
                )*
            }
        }

        macro_rules! merge_opts_fields {
            ($opts:expr, $partial:ident) => {
                $(
                    if let Some(val) = $partial.$field {
                        $opts.$field = val;
                    }
                )*
            }
        }

        pub(crate) use merge_self_opts_fields;
        pub(crate) use merge_opts_fields;
    };
}

define_options! {
    // General
    widget_name: String, "widget_name" => "!table".to_string(),
    widget_id: u32, "widget_id" => generate_unique_id(),
    tree_mode: bool, "tree_mode" => false,
    tree_indent: u64, "tree_indent" => 2,
    empty_val: Val, "empty_val" => Val::Nil(()),
    empty_index_val: Val, "empty_index_val" => Val::Nil(()),
    empty_header_val: Val, "empty_header_val" => Val::Nil(()),
    max_undos: usize, "max_undos" => 50,
    default_row_dim: u64, "default_row_dim" => 40,
    default_col_dim: u64, "default_col_dim" => 150,
    min_row_dim: u64, "min_row_dim" => 1,
    min_col_dim: u64, "min_col_dim" => 1,
    max_row_dim: u64, "max_row_dim" => u64::MAX,
    max_col_dim: u64, "max_col_dim" => u64::MAX,
    paste_expands_y: bool, "paste_expands_y" => false,
    paste_expands_x: bool, "paste_expands_x" => false,
    paste_max_rows: Option<u64>, "paste_max_rows" => None,
    paste_max_cols: Option<u64>, "paste_max_cols" => None,
    csv_out_delimiter: String, "csv_out_delimiter" => "\t".to_string(),
    csv_out_quotechar: String, "csv_out_quotechar" => '"'.to_string(),
    csv_out_lineterminator: String, "csv_out_lineterminator" => "\n".to_string(),
    empty_horizontal: u64, "empty_horizontal" => 50,
    empty_vertical: u64, "empty_vertical" => 50,
    show_horizontal_grid: bool, "show_horizontal_grid" => true,
    show_vertical_grid: bool, "show_vertical_grid" => true,
    table_wrap: String, "table_wrap" => "c".to_string(),
    index_wrap: String, "index_wrap" => "c".to_string(),
    header_wrap: String, "header_wrap" => "c".to_string(),

    // Data formatting default invalid values to display
    invalid_display_un8:   String, "invalid_display_un8"   => "NaN".to_string(),
    invalid_display_un16:  String, "invalid_display_un16"  => "NaN".to_string(),
    invalid_display_un32:  String, "invalid_display_un32"  => "NaN".to_string(),
    invalid_display_un64:  String, "invalid_display_un64"  => "NaN".to_string(),
    invalid_display_un128: String, "invalid_display_un128" => "NaN".to_string(),

    invalid_display_sn8:   String, "invalid_display_sn8"   => "NaN".to_string(),
    invalid_display_sn16:  String, "invalid_display_sn16"  => "NaN".to_string(),
    invalid_display_sn32:  String, "invalid_display_sn32"  => "NaN".to_string(),
    invalid_display_sn64:  String, "invalid_display_sn64"  => "NaN".to_string(),
    invalid_display_sn128: String, "invalid_display_sn128" => "NaN".to_string(),

    invalid_display_fl64:  String, "invalid_display_fl64"  => "NaN".to_string(),
    invalid_display_bigint:String, "invalid_display_bigint"=> "NaN".to_string(),
    invalid_display_bigdec:String, "invalid_display_bigdec"=> "NaN".to_string(),
    invalid_display_dec:   String, "invalid_display_dec"   => "NaN".to_string(),

    invalid_display_str:   String, "invalid_display_str"   => "Invalid".to_string(),
    invalid_display_chr:   String, "invalid_display_chr"   => "?".to_string(),
    invalid_display_bl:    String, "invalid_display_bl"    => "Invalid".to_string(),
    invalid_display_nil:   String, "invalid_display_nil"   => "Invalid".to_string(),
    invalid_display_undf:  String, "invalid_display_undf" => "Invalid".to_string(),
    invalid_display_dt:    String, "invalid_display_dt"    => "Invalid Date".to_string(),
    invalid_display_dur:   String, "invalid_display_dur"   => "Invalid Duration".to_string(),

    // Enabled Functionality
    enabled_undo: bool, "enabled_undo" => false,
    validation_includes_undo: bool, "validation_includes_undo" => false,

    // Appearance
    horizontal_grid_line_width: u64, "horizontal_grid_line_width" => 1,
    vertical_grid_line_width: u64, "vertical_grid_line_width" => 1,
    show_row_numbering: String, "show_row_numbering" => "numbers".to_string(),
    show_col_numbering: String, "show_col_numbering" => "letters".to_string(),
    show_index: bool, "show_index" => true,
    show_header: bool, "show_header" => true,
    popup_menu_fg: Color3, "popup_menu_fg" => Color3 {r: 0, g: 0, b: 0},
    popup_menu_bg: Color3, "popup_menu_bg" => Color3 {r: 255, g: 255, b: 255},
    popup_menu_highlight_bg: Color3, "popup_menu_highlight_bg" => Color3 {r: 220, g: 222, b: 224},
    popup_menu_highlight_fg: Color3, "popup_menu_highlight_fg" => Color3 {r: 0, g: 0, b: 0},
    index_hidden_rows_expander_bg: Color3, "index_hidden_rows_expander_bg" => Color3 {r: 116, g: 119, b: 117},
    header_hidden_columns_expander_bg: Color3, "header_hidden_columns_expander_bg" => Color3 {r: 116, g: 119, b: 117},
    header_bg: Color3, "header_bg" => Color3 {r: 255, g: 255, b: 255},
    header_border_fg: Color3, "header_border_fg" => Color3 {r: 196, g: 199, b: 197},
    header_grid_fg: Color3, "header_grid_fg" => Color3 {r: 196, g: 199, b: 197},
    header_fg: Color3, "header_fg" => Color3 {r: 68, g: 71, b: 70},
    header_editor_bg: Color3, "header_editor_bg" => Color3 {r: 255, g: 255, b: 255},
    header_editor_fg: Color3, "header_editor_fg" => Color3 {r: 68, g: 71, b: 70},
    header_editor_select_bg: Color3, "header_editor_select_bg" => Color3 {r: 207, g: 209, b: 209},
    header_editor_select_fg: Color3, "header_editor_select_fg" => Color3 {r: 0, g: 0, b: 0},
    header_selected_cells_bg: Color3, "header_selected_cells_bg" => Color3 {r: 211, g: 227, b: 253},
    header_selected_cells_fg: Color3, "header_selected_cells_fg" => Color3 {r: 0, g: 0, b: 0},
    index_bg: Color3, "index_bg" => Color3 {r: 255, g: 255, b: 255},
    index_border_fg: Color3, "index_border_fg" => Color3 {r: 196, g: 199, b: 197},
    index_grid_fg: Color3, "index_grid_fg" => Color3 {r: 196, g: 199, b: 197},
    index_fg: Color3, "index_fg" => Color3 {r: 0, g: 0, b: 0},
    index_editor_bg: Color3, "index_editor_bg" => Color3 {r: 255, g: 255, b: 255},
    index_editor_fg: Color3, "index_editor_fg" => Color3 {r: 0, g: 0, b: 0},
    index_editor_select_bg: Color3, "index_editor_select_bg" => Color3 {r: 207, g: 209, b: 209},
    index_editor_select_fg: Color3, "index_editor_select_fg" => Color3 {r: 0, g: 0, b: 0},
    index_selected_cells_bg: Color3, "index_selected_cells_bg" => Color3 {r: 211, g: 227, b: 253},
    index_selected_cells_fg: Color3, "index_selected_cells_fg" => Color3 {r: 0, g: 0, b: 0},
    top_left_bg: Color3, "top_left_bg" => Color3 {r: 249, g: 251, b: 253},
    top_left_fg: Color3, "top_left_fg" => Color3 {r: 217, g: 217, b: 217},
    top_left_fg_highlight: Color3, "top_left_fg_highlight" => Color3 {r: 116, g: 119, b: 117},
    table_bg: Color3, "table_bg" => Color3 {r: 255, g: 255, b: 255},
    table_grid_fg: Color3, "table_grid_fg" => Color3 {r: 225, g: 225, b: 225},
    table_fg: Color3, "table_fg" => Color3 {r: 0, g: 0, b: 0},
    table_editor_bg: Color3, "table_editor_bg" => Color3 {r: 255, g: 255, b: 255},
    table_editor_fg: Color3, "table_editor_fg" => Color3 {r: 0, g: 0, b: 0},
    table_editor_select_bg: Color3, "table_editor_select_bg" => Color3 {r: 207, g: 209, b: 209},
    table_editor_select_fg: Color3, "table_editor_select_fg" => Color3 {r: 0, g: 0, b: 0},
    table_selected_box_cells_fg: Color3, "table_selected_box_cells_fg" => Color3 {r: 11, g: 87, b: 208},
    table_selected_box_rows_fg: Color3, "table_selected_box_rows_fg" => Color3 {r: 11, g: 87, b: 208},
    table_selected_box_columns_fg: Color3, "table_selected_box_columns_fg" => Color3 {r: 11, g: 87, b: 208},
    table_selected_cells_border_fg: Color3, "table_selected_cells_border_fg" => Color3 {r: 11, g: 87, b: 208},
    table_selected_cells_bg: Color3, "table_selected_cells_bg" => Color3 {r: 230, g: 239, b: 253},
    table_selected_cells_fg: Color3, "table_selected_cells_fg" => Color3 {r: 0, g: 0, b: 0},
    resizing_line_fg: Color3, "resizing_line_fg" => Color3 {r: 0, g: 0, b: 0},
    drag_and_drop_bg: Color3, "drag_and_drop_bg" => Color3 {r: 11, g: 87, b: 208},
    header_selected_columns_bg: Color3, "header_selected_columns_bg" => Color3 {r: 11, g: 87, b: 208},
    header_selected_columns_fg: Color3, "header_selected_columns_fg" => Color3 {r: 255, g: 255, b: 255},
    index_selected_rows_bg: Color3, "index_selected_rows_bg" => Color3 {r: 11, g: 87, b: 208},
    index_selected_rows_fg: Color3, "index_selected_rows_fg" => Color3 {r: 255, g: 255, b: 255},
    table_selected_rows_border_fg: Color3, "table_selected_rows_border_fg" => Color3 {r: 11, g: 87, b: 208},
    table_selected_rows_bg: Color3, "table_selected_rows_bg" => Color3 {r: 230, g: 239, b: 253},
    table_selected_rows_fg: Color3, "table_selected_rows_fg" => Color3 {r: 0, g: 0, b: 0},
    table_selected_columns_border_fg: Color3, "table_selected_columns_border_fg" => Color3 {r: 11, g: 87, b: 208},
    table_selected_columns_bg: Color3, "table_selected_columns_bg" => Color3 {r: 230, g: 239, b: 253},
    table_selected_columns_fg: Color3, "table_selected_columns_fg" => Color3 {r: 0, g: 0, b: 0},
    tree_arrow_fg: Color3, "tree_arrow_fg" => Color3 {r: 0, g: 0, b: 0},
    selected_cells_tree_arrow_fg: Color3, "selected_cells_tree_arrow_fg" => Color3 {r: 0, g: 0, b: 0},
    selected_rows_tree_arrow_fg: Color3, "selected_rows_tree_arrow_fg" => Color3 {r: 255, g: 255, b: 255},
}

impl Table {
    pub fn acr_validate_partial_opts(partial: &mut PartialOptions) -> Result<(), String> {
        // TODO: Check the incoming settings are valid. Perhaps return Err if not?
        Ok(())
    }

    /// Validates input before applying settings changes to table.
    pub fn cr_merge_opts(&mut self, mut partial: PartialOptions) -> Result<(), String> {
        Self::acr_validate_partial_opts(&mut partial)?;
        merge_self_opts_fields!(self, partial);
        Ok(())
    }
}

impl Options {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge(mut merge_into: Options, merge_using: PartialOptions) -> Options {
        merge_opts_fields!(merge_into, merge_using);
        merge_into
    }
}

impl PartialOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
