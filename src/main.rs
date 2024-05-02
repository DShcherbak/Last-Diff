use std::env;

use tree_sitter::Parser;
use crate::parse::parse_file;
use crate::tree_compare::*;
use crate::diffs::create_diffs;
use crate::rules::*;

pub mod print;
pub mod parse;
pub mod tree_compare;
pub mod diffs;
pub mod rules;



fn main() {
    //   println!("Hello, Last!");
    let _args: Vec<String> = env::args().collect();
    let before_file = "src/tests/before-1.cpp"; //&args[1];
    let after_file = "src/tests/after-1.cpp"; //&args[2];
    let mut parser = Parser::new();
    let before_content = parse_file(before_file);
    let after_content = parse_file(after_file);
    let mut rules : Option<Rules> = None;
    if before_file.ends_with(".cpp") { //todo normal language detection
        parser.set_language(&tree_sitter_cpp::language()).unwrap();
        rules = Some(set_rules());
    } else if before_file.ends_with(".ml") {
        parser.set_language(&tree_sitter_ocaml::language_ocaml()).unwrap();
    } else {
        let current_diff = difflib::unified_diff(
            &before_content.split('\n').collect::<Vec<&str>>(),
            &after_content.split('\n').collect::<Vec<&str>>(),
            "before",
            "after",
            "",
            "",
            3,
        );
        for line in &current_diff {
            println!("{}", line);
        }
    }
    let before_tree = parser.parse(before_content.clone(), None).unwrap();
    let after_tree = parser.parse(after_content.clone(), None).unwrap();
    // let _ = fs::write("before_tree.txt", before_tree.root_node().to_sexp());
    //   let _ = fs::write("after_tree.txt", after_tree.root_node().to_sexp());
    //    print_tree(&before_content, &before_tree);
    //    println!("-----------------");
    //    print_tree(&after_content, &after_tree);

    //   println!("-----------------");
    if let Some(rules) = rules {
        let before_highs = parse_highs(&before_tree, &rules);
        let after_highs = parse_highs(&after_tree, &rules);
        let diffs = compare_highs(before_highs, after_highs);
        let diff_strings = create_diffs(diffs, &before_content, &after_content);
        if diff_strings.len() == 0 {
            println!("\n\t\t\tNo structural diff between files.\n\n");
        }
        for diff in diff_strings {
            print!("{}", diff);
        } 
    } else {
        if before_tree
        .root_node()
        .to_sexp()
        .eq(&after_tree.root_node().to_sexp())
        {
            println!("\n\t\t\tNo structural diff between files.\n\n");
        } else {
            let diffs = compare_trees_on_high_level(&before_tree, &after_tree);
            let diff_strings = create_diffs(diffs, &before_content, &after_content);
            for diff in diff_strings {
                print!("{}", diff);
            }
        }
    }
}

