use tree_sitter::{Language, Parser, Point};
use std::env;
use std::fs;

extern "C" { fn tree_sitter_cpp() -> Language; }

fn parse_file(file_path : &str) -> String {

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

   // println!("With text:\n{contents}");
    contents
}

fn print_tree(src: &str, tree: &tree_sitter::Tree) {
    let mut cursor = tree.walk();
    print_cursor(src, &mut cursor, 0);
}

fn print_cursor(src: &str, cursor: &mut tree_sitter::TreeCursor, depth: usize) {
    loop {
        let node = cursor.node();
        node.end_position();

        let formatted_node = format!(
            "{}",
            node.kind().replace('\n', "\\n"),
        );

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

fn compare_trees_on_high_level(left_tree: &tree_sitter::Tree, right_tree: &tree_sitter::Tree) {
    let l_cursor = &mut left_tree.walk();
    let r_cursor = &mut right_tree.walk();

    

}

fn main() {
    println!("Hello, Last!");
    let _args: Vec<String> = env::args().collect();
    let before_file = "before.cpp"; //&args[1];
    let after_file = "after.cpp";//&args[2];
    let mut parser = Parser::new();
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

    println!("-----------------");
    println!("Are equal? {:?}", before_tree.root_node().to_sexp().eq(&after_tree.root_node().to_sexp()));
}

#[test]
fn test_parser() {
    let language = unsafe { tree_sitter_cpp() };
    let mut parser = Parser::new();
   // parser.set_language(&tree_sitter_rust::language()).unwrap();
    parser.set_language(&language).unwrap();

    // Store some source code in an array of lines.
    let lines = &[
        "int main() {",
        "  return 0;",
        "}",
    ];

    // Parse the source code using a custom callback. The callback is called
    // with both a byte offset and a row/column offset.
    let tree = parser.parse_with(&mut |_byte: usize, position: Point| -> &[u8] {
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
    }, None).unwrap();

    assert_eq!(
    tree.root_node().to_sexp(),
    "(translation_unit (function_definition type: (primitive_type) declarator: (function_declarator declarator: (identifier) parameters: (parameter_list)) body: (compound_statement (return_statement (number_literal)))))"
    );
}