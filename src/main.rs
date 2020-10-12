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
fn main() {

}