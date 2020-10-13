//! Declaration of the syntax tokens and lexer implementation.

#![allow(non_camel_case_types)]

use logos::{Lexer, Logos};

/// Enum containing all the tokens in a syntax tree.
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    #[regex(r"([ \t])+")]
    WHITESPACE = 0,

    // #[regex(r"(\n|\r\n)+")]
    // NEWLINE,

    #[regex(r"//[^\n\r]*")]
    COMMENT,

    #[regex(r"[A-Za-z0-9_-]+")]
    IDENT,

    #[token("[")]
    BRACKET_START,

    #[token("]")]
    BRACKET_END,

    #[token("{")]
    BRACE_START,

    #[token("}")]
    BRACE_END,

    #[token(",")]
    COMMA,

    #[token(":")]
    COLON,

    #[regex(r#"""#, lex_string_literal)]
    STRING_LITERAL,
    //
    // #[regex(r#"'''"#, lex_multi_line_string_literal)]
    // MULTI_LINE_STRING_LITERAL,

    #[regex(r"[+-]?[0-9_]+", priority = 3)]
    INTEGER,

    #[regex(r"[-+]?([0-9_]+(\.[0-9_]+)?([eE][+-]?[0-9_]+)?|nan|inf)", priority = 2)]
    FLOAT,

    #[regex(r"true|false")]
    BOOL,

    // Good luck debugging this
    // #[regex(r"(([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])[Tt ]([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?(([Zz])|([\+|\-]([01][0-9]|2[0-3]):[0-5][0-9]))?|([0-9]+)-(0[1-9]|1[012])-(0[1-9]|[12][0-9]|3[01])|([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]|60)(\.[0-9]+)?)")]
    // DATE,

    #[error]
    ERROR,

    // composite types
    // 'id', 'type', ...
    KEY,
    // '1', 'document', ...
    VALUE,
    // key = "value"
    ENTRY,
    // [1,2,]
    ARRAY,
    // {}
    DICT,
    // root node
    ROOT,
}

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

pub type SyntaxNode = rowan::SyntaxNode<Lang>;
pub type SyntaxToken = rowan::SyntaxToken<Lang>;
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
