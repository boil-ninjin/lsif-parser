use rowan::SmolStr;
/// Let's start with defining all kinds of tokens and
/// composite nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
enum SyntaxKind {
    L_PAREN = 0, // '{'
    R_PAREN,     // '}'
    COMMA,       // ','
    COLON,   // ':'
    WORD,        // '+', '15'
    WHITESPACE,  // whitespaces is explicit
    ERROR,       // as well as errors

    // composite nodes
    LIST, // `(+ 2 3)`
    ATOM, // `+`, `15`, wraps a WORD token
    ROOT, // top-level node: a list of s-expressions
}
use SyntaxKind::*;

fn test() {

}
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let tarou = Person { name: "太郎".to_string(), age: 18 };
    let json = serde_json::to_string(&tarou).unwrap();
    println!("{}", json);

    // ここが追加分
    let json = r#"{ "name": "花子", "age": 68 }"#;
    let hanako: Person = serde_json::from_str(json).unwrap();
    println!("{:?}", hanako);

}
