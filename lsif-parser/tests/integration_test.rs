use std::fs;
use lsif_parser::parser;

#[test]
fn test() {
    // let root = parser::parse("{id:1, test:\"value\"}");
    let root = parser::parse("{ id: 1, test: \"value\"}\n    {id: 2, test: \"value\"}");
    for e in &root.errors {
        println!("{:?}", e);
    }
    let syntax = root.into_syntax();
    assert_eq!(format!("{:?}", &syntax), "ROOT@0..50");
    // let children = &syntax
    let children = syntax.first_child().unwrap()
        // .next_sibling().unwrap()
        .children_with_tokens()
        .map(|child| format!("{:?}@{:?}", child.kind(), child.text_range()))
        .collect::<Vec<_>>();
    assert_eq!(
        children,
        vec![
            "BRACE_START@0..1",
            "WHITESPACE@1..2",
            "KEY@2..4",
            "COLON@4..5",
            "WHITESPACE@5..6",
            "VALUE@6..7",
            "COMMA@7..8",
            "WHITESPACE@8..9",
            "KEY@9..13",
            "COLON@13..14",
            "WHITESPACE@14..15",
            "VALUE@15..22",
            "BRACE_END@22..23"
        ]
    );
}

// #[test]
// fn test_real() {
//     let str = fs::read_to_string("../samples/jsonRPC.lsif").unwrap();
//     let root = parser::parse(&str);
//     let syntax = root.into_syntax();
//     assert_eq!(format!("{:?}", syntax), "ROOT@0..5384658");
// }