use std::time::Instant;
use tree_table_core::{
    PartialOptions, Table, Val,
    args::{AddArgs, TreeExtendArgs, TreeExtendRow},
};

// fn generate_large_tree(num_nodes: usize) -> Vec<TreeExtendRow> {
//     let mut rows = vec![];
//     let mut counter = 0;
//     let root_iid = counter.to_string();
//     while counter < num_nodes {
//         rows.push(TreeExtendRow {
//             iid: format!("{root_iid}.{counter}"),
//             par: "".to_string(),
//             val: None,
//             vals: Some(vec![
//                 ("col1".to_string(), Val::Str("yo".to_string())),
//                 ("col2".to_string(), Val::Str("yo".to_string())),
//             ]),
//             open: false,
//             dim: None,
//         });
//         counter += 1;
//     }
//     rows
// }

fn generate_large_tree(num_nodes: usize) -> Vec<TreeExtendRow> {
    let mut rows = vec![];
    let mut counter = 0;
    let root_iid = counter.to_string();
    rows.push(TreeExtendRow {
        iid: root_iid.clone(),
        par: "".to_string(),
        val: None,
        vals: Some(vec![("col1".to_string(), Val::Str("hi".to_string()))]),
        open: false,
        dim: None,
    });
    counter += 1;

    while counter < num_nodes {
        let iid = counter.to_string();
        rows.push(TreeExtendRow {
            iid: iid.clone(),
            par: root_iid.clone(),
            val: None,
            vals: Some(vec![("col1".to_string(), Val::Str("hi".to_string()))]),
            open: true,
            dim: None,
        });

        let x_iid = format!("{}x", iid);
        rows.push(TreeExtendRow {
            iid: x_iid.clone(),
            par: iid.clone(),
            val: None,
            vals: Some(vec![("col1".to_string(), Val::Str("hi".to_string()))]),
            open: true,
            dim: None,
        });

        let y_iid = format!("{}y", iid);
        rows.push(TreeExtendRow {
            iid: y_iid.clone(),
            par: x_iid.clone(),
            val: None,
            vals: Some(vec![("col1".to_string(), Val::Str("hi".to_string()))]),
            open: true,
            dim: None,
        });

        let a_iid = format!("{}a", iid);
        rows.push(TreeExtendRow {
            iid: a_iid.clone(),
            par: y_iid.clone(),
            val: None,
            vals: Some(vec![("col1".to_string(), Val::Str("hi".to_string()))]),
            open: true,
            dim: None,
        });

        let b_iid = format!("{}b", iid);
        rows.push(TreeExtendRow {
            iid: b_iid.clone(),
            par: a_iid.clone(),
            val: None,
            vals: Some(vec![("col1".to_string(), Val::Str("hi".to_string()))]),
            open: true,
            dim: None,
        });

        let c_iid = format!("{}c", iid);
        rows.push(TreeExtendRow {
            iid: c_iid.clone(),
            par: b_iid.clone(),
            val: None,
            vals: Some(vec![("col1".to_string(), Val::Str("hi".to_string()))]),
            open: true,
            dim: None,
        });

        let d_iid = format!("{}d", iid);
        rows.push(TreeExtendRow {
            iid: d_iid.clone(),
            par: c_iid.clone(),
            val: None,
            vals: Some(vec![("col1".to_string(), Val::Str("hi".to_string()))]),
            open: true,
            dim: None,
        });

        let e_iid = format!("{}e", iid);
        let f_iid = format!("{}f", iid);
        rows.push(TreeExtendRow {
            iid: e_iid.clone(),
            par: f_iid.clone(),
            val: None,
            vals: Some(vec![("col1".to_string(), Val::Str("hi".to_string()))]),
            open: true,
            dim: None,
        });

        let g_iid = format!("{}g", iid);
        rows.push(TreeExtendRow {
            iid: f_iid.clone(),
            par: g_iid.clone(),
            val: None,
            vals: Some(vec![("col1".to_string(), Val::Str("hi".to_string()))]),
            open: true,
            dim: None,
        });

        let h_iid = format!("{}h", iid);
        rows.push(TreeExtendRow {
            iid: g_iid.clone(),
            par: h_iid.clone(),
            val: None,
            vals: Some(vec![("col1".to_string(), Val::Str("hi".to_string()))]),
            open: true,
            dim: None,
        });

        counter += 1;
    }
    rows
}

fn main() {
    let table = Table::new(
        30,
        Some(PartialOptions {
            tree_mode: Some(true),
            enabled_undo: Some(true),
            ..Default::default()
        }),
    );
    table
        .add_cols(Some(AddArgs {
            items: Some(vec!["col1".to_string(), "col2".to_string()]),
            ..Default::default()
        }))
        .ok();

    let rows = generate_large_tree(1);
    let args = TreeExtendArgs {
        rows,
        ..Default::default()
    };

    let start = Instant::now();
    if let Err(e) = table.tree_extend(Some(args)) {
        println!("Add rows failed: {}", e);
    }
    let duration = start.elapsed();
    println!("Time taken for tree_extend: {:?}", duration);
    println!("Total rows: {:?}", table.total_rows());
}
