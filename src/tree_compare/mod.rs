use tree_sitter::TreeCursor;

#[derive(PartialEq)]
pub struct DiffCheckCoord {
    pub start_byte_left: usize,
    pub end_byte_left: usize,
    pub start_byte_right: usize,
    pub end_byte_right: usize,
}

fn compare_trees_walk(left: &mut TreeCursor, right: &mut TreeCursor) -> Vec<DiffCheckCoord> {
    let l_node = left.node();
    let r_node = right.node();
    let mut diff_coords: Vec<DiffCheckCoord> = Vec::new();
   // println!("left node: {}, {}", l_node.kind(), l_node.child_count());
  //  println!("right node: {}, {}", r_node.kind(), r_node.child_count());
    if l_node.kind() != r_node.kind() || l_node.child_count() != r_node.child_count() {
        return vec![DiffCheckCoord {
                start_byte_left: l_node.start_byte(),
                end_byte_left: l_node.end_byte(),
                start_byte_right: r_node.start_byte(),
                end_byte_right: r_node.end_byte(),
            }];
    }
    if l_node.child_count() == 0 {
        if l_node.end_byte() - l_node.start_byte() != r_node.end_byte() - r_node.start_byte() {
            diff_coords.push(DiffCheckCoord {
                start_byte_left: l_node.start_byte(),
                end_byte_left: l_node.end_byte(),
                start_byte_right: r_node.start_byte(),
                end_byte_right: r_node.end_byte(),
            });
        }
        return diff_coords;
    }
    left.goto_first_child();
    right.goto_first_child();
    for _ in 0..l_node.child_count() {
        let child_diff = compare_trees_walk(left, right);
        if child_diff.len() > 0 {
            diff_coords.extend(child_diff);
        }
        left.goto_next_sibling();
        right.goto_next_sibling();
    }
    left.goto_parent();
    right.goto_parent();
    diff_coords
}

pub fn compare_trees_on_high_level(
    left_tree: &tree_sitter::Tree,
    right_tree: &tree_sitter::Tree,
) -> Vec<DiffCheckCoord> {
    let l_cursor = &mut left_tree.walk();
    let r_cursor = &mut right_tree.walk();
    return compare_trees_walk(l_cursor, r_cursor);
}

pub fn compare_nodes_on_high_level(
    left_tree: &tree_sitter::Node,
    right_tree: &tree_sitter::Node,
) -> Vec<DiffCheckCoord> {
    let l_cursor = &mut left_tree.walk();
    let r_cursor = &mut right_tree.walk();
    return compare_trees_walk(l_cursor, r_cursor);
}

pub fn compare_highs(mut before_highs: Vec<tree_sitter::Node>, mut after_highs: Vec<tree_sitter::Node>) -> Vec<DiffCheckCoord> {
    let mut diff_coords: Vec<DiffCheckCoord> = Vec::new();
    let mut pairs: Vec<(usize, usize, bool)> = Vec::new();
    for i in 0..before_highs.len() {
        let before_node = before_highs[i];
        'j: for j in 0..after_highs.len() {
            let after_node = after_highs[j];
            if before_node.kind() == after_node.kind() {
                if compare_nodes_on_high_level(&before_node, &after_node) == vec![] {
                    pairs.push((i, j, true));
                    break 'j;
                }
                let mut next_kin_before = before_node.walk();
                let mut next_kin_after = after_node.walk();
                next_kin_before.goto_first_child();
                next_kin_after.goto_first_child();
                if compare_trees_walk(&mut next_kin_before.clone(), &mut next_kin_after.clone()) != vec![] {
                    continue; 
                }
                next_kin_before.goto_next_sibling();
                next_kin_after.goto_next_sibling();
                if compare_trees_walk(&mut next_kin_before.clone(), &mut next_kin_after.clone()) != vec![] {
                    continue;
                }
                pairs.push((i, j, false));
            }
        }
    }
    let mut removed:Vec<bool> = before_highs.clone().iter().map(|hi| true).collect();
    let mut inserted:Vec<bool> = before_highs.clone().iter().map(|hi| true).collect();
   // println!("pairs: {:?}", pairs);
   // println!("removed: {:?}", removed);
    for pair in pairs {
        removed[pair.0] = false;
        inserted[pair.1] = false;

        if !pair.2 {
            diff_coords.push(DiffCheckCoord {
                start_byte_left: before_highs[pair.0].start_byte(),
                end_byte_left: before_highs[pair.0].end_byte(),
                start_byte_right: after_highs[pair.1].start_byte(),
                end_byte_right: after_highs[pair.1].end_byte(),
            });
        }
    }
    for (i, r) in removed.iter().enumerate() {
        if *r {
            diff_coords.push(DiffCheckCoord {
                start_byte_left: before_highs[i].start_byte(),
                end_byte_left: before_highs[i].end_byte(),
                start_byte_right: 0,
                end_byte_right: 0,
            });
        
        }
    }
    for (i, r) in inserted.iter().enumerate() {
        if *r {
            diff_coords.push(DiffCheckCoord {
                start_byte_left: 0,
                end_byte_left: 0,
                start_byte_right: after_highs[i].start_byte(),
                end_byte_right: after_highs[i].end_byte(),
            });
        }
    }
    diff_coords
}
