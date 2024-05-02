pub struct Rules {
    pub head:String,
    pub high_level_rules: Vec<String>
}

pub fn set_rules() -> Rules {
    Rules {head:"translation_unit".to_string(), 
        high_level_rules:vec!["function_definition".to_string(), "class_specifier".to_string(),]}
}

pub fn parse_highs<'a>(tree: &'a tree_sitter::Tree, rules: &'a Rules) -> Vec<tree_sitter::Node<'a>> {
    let mut high_level_nodes: Vec<tree_sitter::Node> = Vec::new();
    let mut stack: Vec<tree_sitter::Node> = Vec::new();
    let mut stack_rules: Vec<String> = Vec::new();

    if tree.root_node().kind() != rules.head {
        return vec![];
    }
    let cursor = &mut tree.walk();
    cursor.goto_first_child();
    loop {
        if rules.high_level_rules.contains(&cursor.node().kind().to_string()) {
            high_level_nodes.push(cursor.node());
        }
        if !cursor.goto_next_sibling() {
            break;
        }
    }//dont
    stack.push(cursor.node());
    stack_rules.push(rules.head.clone());
    while stack.len() > 0 {
        let current_node = stack.pop().unwrap();
        let current_rule = stack_rules.pop().unwrap();
        cursor.reset(current_node);
        cursor.goto_first_child();
        for _ in 0..current_node.child_count() {
            stack.push(cursor.node());
            stack_rules.push(current_rule.clone());
            cursor.goto_next_sibling();
        }
    }
    high_level_nodes
}