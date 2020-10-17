use lsif_parser_lib::parser;

fn main() {
    let root = parser::parse("\
        { id: 5, type: \"edge\", label: \"contains\", outV: 1, inVs: [4] }");
    // let root = parser::parse("{ id: 1, test: \"value\"}\n\
    //     { id: 4, type: \"vertex\", label: \"range\", start: { line: 0, character: 9}, end: \
    //     { line: 0, character: 12 } }\n    \
    //     { id: 5, type: \"edge\", label: \"contains\", outV: 1, inVs: [4] }");
    for e in &root.errors {
        println!("{:?}", e);
    }
    let syntax = root.into_syntax();
    println!("{:?}", &syntax);
    let children_vec = &syntax
        .children_with_tokens()
        .map(|child| format!("{:?}@{:?}", child.kind(), child.text_range()))
        .collect::<Vec<_>>();
    println!("{:?}", children_vec);
    // let grand_children = &syntax.first_child().unwrap().next_sibling().unwrap().next_sibling().unwrap();
    let grand_children = &syntax.first_child().unwrap();
    let grand_children_vec = &grand_children
        .children_with_tokens()
        .map(|child| format!("{:?}@{:?}", child.kind(), child.text_range()))
        .collect::<Vec<_>>();
    println!("{:?}", grand_children_vec);
}
