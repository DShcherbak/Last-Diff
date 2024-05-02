use tree_sitter::TreeCursor;

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
  //  println!("left node: {}, {}", l_node.kind(), l_node.child_count());
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
