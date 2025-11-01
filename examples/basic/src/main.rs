use std::collections::VecDeque;
use std::time::Instant;

use tree_table_core::{
    PartialOptions, Table,
    args::{TreeExtendArgs, TreeExtendRow},
};

fn generate_tree_rows(depth: usize, root_children: usize) -> Vec<TreeExtendRow> {
    let mut rows = vec![];
    let root_iid = "root".to_string();
    rows.push(TreeExtendRow {
        iid: root_iid.clone(),
        par: "".to_string(),
        val: None,
        vals: None,
        open: false,
        dim: None,
    });

    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    queue.push_back((root_iid, 0));

    let mut counter = 0;
    while !queue.is_empty() {
        let (par, level) = queue.pop_front().unwrap();
        if level >= depth {
            continue;
        }
        let children_count = if level == 0 { root_children } else { 1 };
        for _ in 0..children_count {
            counter += 1;
            let iid = format!("node_{}", counter);
            rows.push(TreeExtendRow {
                iid: iid.clone(),
                par: par.clone(),
                val: None,
                vals: None,
                open: true,
                dim: None,
            });
            queue.push_back((iid, level + 1));
        }
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

    let rows = generate_tree_rows(10, 1000000); // For depth 10, root_children 100000 to get approximately 1,000,001 rows
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
}
