#[test]
fn simple_cpp() {
 //   let language = unsafe { tree_sitter_cpp() };
    let mut parser = Parser::new();
    // parser.set_language(&tree_sitter_rust::language()).unwrap();
    parser.set_language(&tree_sitter_cpp::language()).unwrap();

    // Store some source code in an array of lines.
    let lines = &["int main() {", "  return 0;", "}"];

    // Parse the source code using a custom callback. The callback is called
    // with both a byte offset and a row/column offset.
    let tree = parser
        .parse_with(
            &mut |_byte: usize, position| -> &[u8] {
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


#[test]
fn test_ocaml() {
 //   let language = unsafe { tree_sitter_cpp() };
    let mut parser = Parser::new();
    // parser.set_language(&tree_sitter_rust::language()).unwrap();
    parser.set_language(&tree_sitter_ocaml::language_ocaml()).unwrap();

    // Store some source code in an array of lines.
    let lines = &["let some_func x y = x + 4 ;;"];

    // Parse the source code using a custom callback. The callback is called
    // with both a byte offset and a row/column offset.
    let tree = parser
        .parse_with(
            &mut |_byte: usize, position| -> &[u8] {
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
