use std::env;
use std::fs;
use tree_sitter::TreeCursor;
use tree_sitter::Parser;


fn parse_file(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");
    contents
}

#[allow(dead_code)]
fn print_tree(src: &str, tree: &tree_sitter::Tree) {
    let mut cursor = tree.walk();
    print_cursor(src, &mut cursor, 0);
}

#[allow(dead_code)]
fn print_cursor(src: &str, cursor: &mut tree_sitter::TreeCursor, depth: usize) {
    loop {
        let node = cursor.node();

        let formatted_node = format!("{}", node.kind().replace('\n', "\\n"),);

        if node.child_count() == 0 {
            let node_src = &src[node.start_byte()..node.end_byte()];
            println!("{}{} {:?}", "  ".repeat(depth), formatted_node, node_src);
        } else {
            println!("{}{}", "  ".repeat(depth), formatted_node,);
        }

        if cursor.goto_first_child() {
            print_cursor(src, cursor, depth + 1);
            cursor.goto_parent();
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

struct DiffCheckCoord {
    start_byte_left: usize,
    end_byte_left: usize,
    start_byte_right: usize,
    end_byte_right: usize,
}

fn compare_trees_walk(left: &mut TreeCursor, right: &mut TreeCursor) -> Vec<DiffCheckCoord> {
    let l_node = left.node();
    let r_node = right.node();
    let mut diff_coords: Vec<DiffCheckCoord> = Vec::new();
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

fn compare_trees_on_high_level(
    left_tree: &tree_sitter::Tree,
    right_tree: &tree_sitter::Tree,
) -> Vec<DiffCheckCoord> {
    let l_cursor = &mut left_tree.walk();
    let r_cursor = &mut right_tree.walk();
    return compare_trees_walk(l_cursor, r_cursor);
}

fn create_diffs(diffs: Vec<DiffCheckCoord>, left_file: &str, right_file: &str) -> Vec<String> {
    if diffs.len() == 0 {
        return vec!["\n\t\t\tNo structural diff found between files.\n".to_string()];
    }
    let mut diff_strings: Vec<String> = Vec::new();
    for diff in diffs {
        let left_diff = &left_file[diff.start_byte_left..diff.end_byte_left]
            .split('\n')
            .collect::<Vec<&str>>();
        let right_diff = &right_file[diff.start_byte_right..diff.end_byte_right]
            .split('\n')
            .collect::<Vec<&str>>();
        diff_strings.extend(difflib::unified_diff(
            &left_diff,
            &right_diff,
            "left",
            "right",
            "L",
            "R",
            3,
        ));
        diff_strings.push("-------------------------\n-------------------------\n".to_string());
    }
    diff_strings
}

fn main() {
    //   println!("Hello, Last!");
    let _args: Vec<String> = env::args().collect();
    let before_file = "src/tests/before.ml"; //&args[1];
    let after_file = "src/tests/after.ml"; //&args[2];
    let mut parser = Parser::new();
    // parser.set_language(&tree_sitter_ocaml::language_ocaml()).unwrap();
    parser.set_language(&tree_sitter_cpp::language()).unwrap();
    let before_content = parse_file(before_file);
    let after_content = parse_file(after_file);
    let before_tree = parser.parse(before_content.clone(), None).unwrap();
    let after_tree = parser.parse(after_content.clone(), None).unwrap();
    // let _ = fs::write("before_tree.txt", before_tree.root_node().to_sexp());
    //   let _ = fs::write("after_tree.txt", after_tree.root_node().to_sexp());
       print_tree(&before_content, &before_tree);
       println!("-----------------");
       print_tree(&after_content, &after_tree);

    //   println!("-----------------");
    if before_tree
        .root_node()
        .to_sexp()
        .eq(&after_tree.root_node().to_sexp())
    {
        println!("\n\t\t\tNo structural diff between files.\n");
    } else {
        let diffs = compare_trees_on_high_level(&before_tree, &after_tree);
        let diff_strings = create_diffs(diffs, &before_content, &after_content);
        for diff in diff_strings {
            println!("{}", diff);
        }
    }
}

#[test]
fn test_parser() {
    let language = unsafe { tree_sitter_cpp() };
    let mut parser = Parser::new();
    // parser.set_language(&tree_sitter_rust::language()).unwrap();
    parser.set_language(&language).unwrap();

    // Store some source code in an array of lines.
    let lines = &["int main() {", "  return 0;", "}"];

    // Parse the source code using a custom callback. The callback is called
    // with both a byte offset and a row/column offset.
    let tree = parser
        .parse_with(
            &mut |_byte: usize, position: Point| -> &[u8] {
                let row = position.row as usize;
                let column = position.column as usize;
                if row < lines.len() {
                    if column < lines[row].as_bytes().len() {
                        &lines[row].as_bytes()[column..]
                    } else {
                        b"\n"
                    }
                } else {
                    &[]
                }
            },
            None,
        )
        .unwrap();

    assert_eq!(
    tree.root_node().to_sexp(),
    "(translation_unit (function_definition type: (primitive_type) declarator: (function_declarator declarator: (identifier) parameters: (parameter_list)) body: (compound_statement (return_statement (number_literal)))))"
    );
}
