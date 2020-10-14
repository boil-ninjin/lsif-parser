use lsif_parser_lib::parser;

fn main() {
    println!("Hello, world!");
    let sexps = "
92
(+ 62 30)
(/ 92 0)
nan
(+ (* 15 2) 62)
";
    let root = parser::parse(sexps).root();
    let res = root.sexps().map(|it| it.eval()).collect::<Vec<_>>();
    eprintln!("{:?}", res);
    assert_eq!(res, vec![Some(92), Some(92), None, None, Some(92),])
}
