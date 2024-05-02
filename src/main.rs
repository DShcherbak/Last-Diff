use std::env;

use tree_sitter::Parser;
use crate::parse::parse_file;
use crate::tree_compare::{compare_trees_on_high_level, DiffCheckCoord};
use crate::diffs::create_diffs;

pub mod print;
pub mod parse;
pub mod tree_compare;
pub mod diffs;



fn main() {
    //   println!("Hello, Last!");
    let _args: Vec<String> = env::args().collect();
    let before_file = "src/tests/before.ml"; //&args[1];
    let after_file = "src/tests/after.ml"; //&args[2];
    let mut parser = Parser::new();
    // parser.set_language(&tree_sitter_ocaml::language_ocaml()).unwrap();
    parser.set_language(&tree_sitter_ocaml::language_ocaml()).unwrap();
    let before_content = parse_file(before_file);
    let after_content = parse_file(after_file);
    let before_tree = parser.parse(before_content.clone(), None).unwrap();
    let after_tree = parser.parse(after_content.clone(), None).unwrap();
    // let _ = fs::write("before_tree.txt", before_tree.root_node().to_sexp());
    //   let _ = fs::write("after_tree.txt", after_tree.root_node().to_sexp());
    //    print_tree(&before_content, &before_tree);
    //    println!("-----------------");
    //    print_tree(&after_content, &after_tree);

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
            print!("{}", diff);
        }
    }
}

