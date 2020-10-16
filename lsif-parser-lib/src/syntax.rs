//! Declaration of the syntax tokens and lexer implementation.

#![allow(non_camel_case_types)]

use logos::{Lexer, Logos};

/// Let's start with defining all kinds of tokens and
/// composite nodes.
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    #[regex(r"([ \t])+")]
    WHITESPACE = 0,
    // NEWLINE may be explicit
    #[regex(r"(\n|\r\n)+")]
    NEWLINE,
    #[regex(r"//[^\n\r]*")]
    COMMENT,
    // may be keys in entry
    #[regex(r"[A-Za-z]+")]
    IDENT,
    // divide each entry in sentence
    #[token(",")]
    COMMA,
    #[token(":")]
    COLON,
    #[regex(r#"""#, lex_string)]
    STRING,
    #[regex(r"[+-]?[0-9_]+", priority = 2)]
    INTEGER,
    #[regex(r"true|false")]
    BOOL,

    #[token("[")]
    BRACKET_START,

    #[token("]")]
    BRACKET_END,

    #[token("{")]
    BRACE_START,

    #[token("}")]
    BRACE_END,

    #[error]
    ERROR,       // as well as errors

    // composite nodes
    KEY,
    VALUE,
    VERTEX,
    // e.g.: {key:{ inline_key:"inline_value" }}
    INLINE,
    // e.g.: [0,2,4]
    LIST,
    // one key-value pair
    ENTRY,
    // one sentence
    SENTENCE,
    // top-level node: a list of s-expressions
    ROOT,
}

/// Some boilerplate is needed, as rowan settled on using its own
/// `struct SyntaxKind(u16)` internally, instead of accepting the
/// user's `enum SyntaxKind` as a type parameter.
///
/// First, to easily pass the enum variants into rowan via `.into()`:
impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

/// Second, implementing the `Language` trait teaches rowan to convert between
/// these two SyntaxKind types, allowing for a nicer SyntaxNode API where
/// "kinds" are values from our `enum SyntaxKind`, instead of plain u16 values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}

impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

/// To work with the parse results we need a view into the
/// green tree - the Syntax tree.
/// It is also immutable, like a GreenNode,
/// but it contains parent pointers, offsets, and
/// has identity semantics.

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
#[allow(unused)]
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
#[allow(unused)]
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

fn lex_string(lex: &mut Lexer<SyntaxKind>) -> bool {
    let remainder: &str = lex.remainder();
    let mut escaped = false;

    let mut total_len = 0;

    for c in remainder.chars() {
        total_len += c.len_utf8();

        if c == '\\' {
            escaped = !escaped;
            continue;
        }

        if c == '"' && !escaped {
            lex.bump(remainder[0..total_len].as_bytes().len());
            return true;
        }

        escaped = false;
    }
    false
}
