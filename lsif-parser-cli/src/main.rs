use lsif_parser_lib::parser;

fn main() {
    let root = parser::parse("");
    eprintln!("{:?}", root);
}
