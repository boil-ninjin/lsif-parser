use lsif_parser_lib::parser;

fn main() {
    // let root = parser::parse("{id:1, test:\"value\"}");
    let root = parser::parse("{ id: 1, test: \"value\"}\n    {id: 2, test: \"value\"}");
    for e in &root.errors{
        println!("{:?}", e);
    }
    let syntax = root.into_syntax();
    println!("{:?}", &syntax);
    // let children = &syntax
    let children = &syntax.first_child().unwrap()
        // .next_sibling().unwrap()
        .children_with_tokens()
        .map(|child| format!("{:?}@{:?}", child.kind(), child.text_range()))
        .collect::<Vec<_>>();
    println!("{:?}",children);
}
