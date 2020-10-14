use lsif_parser_lib::parser;

fn main() {
    let root = parser::parse(r#"{id:1, test: "value"}"#);
    eprintln!("{:?}", root);
}
