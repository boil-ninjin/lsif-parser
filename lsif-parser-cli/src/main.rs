use lsif_parser_lib::parser;

fn main() {
    let root = parser::parse("{id:1, test: \"value\"}");
    eprintln!("{:?}", root.into_syntax());
}
