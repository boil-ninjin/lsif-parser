//! It's suggested to read the conceptual overview of the design
//! alongside this tutorial:
//! https://github.com/rust-analyzer/rust-analyzer/blob/master/docs/dev/syntax.md

/// Currently, rowan doesn't have a hook to add your own interner,
/// but `SmolStr` should be a "good enough" type for representing
/// tokens.
/// Additionally, rowan uses `TextSize` and `TextRange` types to
/// represent utf8 offsets and ranges.
use crate::{
    dom,
    syntax::{SyntaxKind, SyntaxKind::*, SyntaxNode},
    util::{allowed_chars, check_escape},
};
// use dom::Cast;
use logos::{Lexer, Logos};
use rowan::{GreenNode, GreenNodeBuilder, SmolStr, TextRange, TextSize};
use std::convert::TryInto;

#[macro_use]
mod macros;

/// A syntax error that can occur during parsing.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Error {
    /// The span of the error.
    pub range: TextRange,

    /// Human-friendly error message.
    pub message: String,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?})", &self.message, &self.range)
    }
}

impl std::error::Error for Error {}

/// Parse a LSIF document into a [Rowan green tree](rowan::GreenNode).
///
/// The parsing will not stop at unexpected or invalid tokens.
/// Instead errors will be collected with their character offsets and lengths,
/// and the invalid token(s) will have the `ERROR` kind in the final tree.
///
/// The parser will also validate comment and string contents, looking for
/// invalid escape sequences and invalid characters.
/// These will also be reported as syntax errors.
///
/// This does not check for semantic errors such as duplicate keys.
/// Note that `parse` does not return a `Result`:
/// by design, syntax tree can be built even for
/// completely invalid source code.
pub fn parse(text: &str) -> Parse {
    Parser::new(text).parse()
}

/// A hand-written parser that uses the Logos lexer
/// to tokenize the source, then constructs
/// a Rowan green tree from them.
struct Parser<'p> {
    skip_whitespace: bool,
    current_token: Option<SyntaxKind>,
    // These tokens are not consumed on errors.
    //
    // The syntax error is still reported,
    // but the the surrounding context can still
    // be parsed.
    error_whitelist: u16,
    /// lexer for parse string
    lexer: Lexer<'p, SyntaxKind>,
    /// the in-progress tree.
    builder: GreenNodeBuilder<'p>,
    /// the list of syntax errors we've accumulated
    /// so far.
    errors: Vec<Error>,
}

/// This is just a convenience type during parsing.
/// It allows using "?", making the code cleaner.
type ParserResult<T> = Result<T, ()>;

impl<'p> Parser<'p> {
    fn new(source: &'p str) -> Self {
        Parser {
            current_token: None,
            skip_whitespace: true,
            error_whitelist: 0,
            lexer: SyntaxKind::lexer(source),
            builder: Default::default(),
            errors: Default::default(),
        }
    }
    fn parse(mut self) -> Parse {
        let _ = with_node!(self.builder, ROOT, self.parse_root());

        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }
    fn error(&mut self, message: &str) -> ParserResult<()> {
        let span = self.lexer.span();
        self.add_error(&Error {
            range: TextRange::new(
                TextSize::from(span.start as u32),
                TextSize::from(span.end as u32),
            ),
            message: message.into(),
        });
        if let Some(t) = self.current_token {
            if !self.whitelisted(t) {
                self.token_as(ERROR).ok();
            }
        }
        Err(())
    }

    // report error without consuming the current the token
    fn report_error(&mut self, message: &str) -> ParserResult<()> {
        let span = self.lexer.span();
        self.add_error(&Error {
            range: TextRange::new(
                TextSize::from(span.start as u32),
                TextSize::from(span.end as u32),
            ),
            message: message.into(),
        });
        Err(())
    }

    // add error to errors
    fn add_error(&mut self, e: &Error) {
        if let Some(last_err) = self.errors.last_mut() {
            if last_err == e {
                return;
            }
        }

        self.errors.push(e.clone());
    }

    #[inline]
    fn whitelist_token(&mut self, token: SyntaxKind) {
        self.error_whitelist |= token as u16;
    }

    #[inline]
    fn blacklist_token(&mut self, token: SyntaxKind) {
        self.error_whitelist &= !(token as u16);
    }

    #[inline]
    fn whitelisted(&self, token: SyntaxKind) -> bool {
        self.error_whitelist & token as u16 != 0
    }

    fn insert_token(&mut self, kind: SyntaxKind, s: SmolStr) {
        self.builder.token(kind.into(), s)
    }

    fn must_token_or(&mut self, kind: SyntaxKind, message: &str) -> ParserResult<()> {
        match self.get_token() {
            Ok(t) => {
                if kind == t {
                    self.token()
                } else {
                    self.error(message)
                }
            }
            Err(_) => {
                self.add_error(&Error {
                    range: TextRange::new(
                        self.lexer.span().start.try_into().unwrap(),
                        self.lexer.span().end.try_into().unwrap(),
                    ),
                    message: "unexpected EOF".into(),
                });
                Err(())
            }
        }
    }

    fn token(&mut self) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => Err(()),
            Ok(token) => self.token_as(token),
        }
    }

    fn token_as(&mut self, kind: SyntaxKind) -> ParserResult<()> {
        match self.get_token() {
            Err(_) => return Err(()),
            Ok(_) => {
                self.builder.token(kind.into(), self.lexer.slice().into());
            }
        }

        self.step();
        Ok(())
    }
    fn step(&mut self) {
        self.current_token = None;
        while let Some(token) = self.lexer.next() {
            match token {
                COMMENT => {
                    match allowed_chars::comment(self.lexer.slice()) {
                        Ok(_) => {}
                        Err(err_indices) => {
                            for e in err_indices {
                                self.add_error(&Error {
                                    range: TextRange::new(
                                        (self.lexer.span().start + e).try_into().unwrap(),
                                        (self.lexer.span().start + e).try_into().unwrap(),
                                    ),
                                    message: "invalid character in comment".into(),
                                });
                            }
                        }
                    };

                    self.insert_token(token, self.lexer.slice().into());
                }
                WHITESPACE => {
                    if self.skip_whitespace {
                        self.insert_token(token, self.lexer.slice().into());
                    } else {
                        self.current_token = Some(token);
                        break;
                    }
                }
                ERROR => {
                    self.insert_token(token, self.lexer.slice().into());
                    let span = self.lexer.span();
                    self.add_error(&Error {
                        range: TextRange::new(
                            span.start.try_into().unwrap(),
                            span.end.try_into().unwrap(),
                        ),
                        message: "unexpected token".into(),
                    })
                }
                _ => {
                    self.current_token = Some(token);
                    break;
                }
            }
        }
    }

    fn get_token(&mut self) -> ParserResult<SyntaxKind> {
        if self.current_token.is_none() {
            self.step();
        }

        self.current_token.ok_or(())
    }
    // =============================================================================================
    // Let' parse
    fn parse_root(&mut self) -> ParserResult<()> {
        // We want to make sure that an entry spans the
        // entire line, so we start/close its node manually.
        // let mut entry_started = false;

        while let Ok(token) = self.get_token() {
            match token {
                NEWLINE => {
                    // dispose newline token
                    let _ = self.token();
                }
                _ => {
                    let _ = whitelisted!(self, NEWLINE, !with_node!(self.builder, SENTENCE, self.parse_sentence()));
                }
            }
        }
        if entry_started {
            self.builder.finish_node();
        }
        Ok(())
    }
    fn parse_sentence(&mut self) -> ParserResult<()> {
        self.must_token_or(BRACE_START, r#"expected sentence starts with "{""#)?;
        // count if sentence is finished with brace or not.
        let mut brace_count= 1;
        while brace_count >= 0 {
            let t = match self.get_token() {
                Ok(token) => token,
                Err(_) => {
                    if brace_count == 0{
                        return Ok(());
                    }
                    return self.error("unexpected end of input");
                }
            };
            match t {
                BRACE_END =>{
                    if !finish_sentence {
                        self.token()?;
                        finish_sentence = true;
                    } else {
                        return self.error(r#"unexpected "}""#);
                    }
                }
                COMMA => {
                    if after_period {
                        return self.error(r#"unexpected ".""#);
                    } else {
                        self.token()?;
                        after_period = true;
                    }
                }
                _ => {
                    if after_period {
                        match self.parse_ident() {
                            Ok(_) => {}
                            Err(_) => return self.error("expected identifier"),
                        }
                        after_period = false;
                    } else {
                        break;
                    }
                }
            };
        }

        Ok(())
    }
}

impl Parse {
    fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
    pub fn root(&self) -> Root {
        Root::cast(self.syntax()).unwrap()
    }
}

/// Split the input string into a flat list of tokens
/// (such as L_PAREN, WORD, and WHITESPACE)
fn lex(text: &str) -> Vec<(SyntaxKind, SmolStr)> {
    fn tok(t: SyntaxKind) -> m_lexer::TokenKind {
        m_lexer::TokenKind(rowan::SyntaxKind::from(t).0)
    }
    fn kind(t: m_lexer::TokenKind) -> SyntaxKind {
        match t.0 {
            0 => L_PAREN,
            1 => R_PAREN,
            2 => WORD,
            3 => WHITESPACE,
            4 => ERROR,
            _ => unreachable!(),
        }
    }

    let lexer = m_lexer::LexerBuilder::new()
        .error_token(tok(ERROR))
        .tokens(&[
            (tok(L_PAREN), r"\("),
            (tok(R_PAREN), r"\)"),
            (tok(WORD), r"[^\s()]+"),
            (tok(WHITESPACE), r"\s+"),
        ])
        .build();

    lexer
        .tokenize(text)
        .into_iter()
        .map(|t| (t.len, kind(t.kind)))
        .scan(0usize, |start_offset, (len, kind)| {
            let s: SmolStr = text[*start_offset..*start_offset + len].into();
            *start_offset += len;
            Some((kind, s))
        })
        .collect()
}

/// The parse results are stored as a "green tree".
/// We'll discuss working with the results later
/// The final results of a parsing.
/// It contains the green tree, and
/// the errors that occurred during parsing.
#[derive(Debug, Clone)]
pub struct Parse {
    green_node: GreenNode,
    #[allow(unused)]
    errors: Vec<Error>,
}

impl Parse {
    /// Turn the parse into a syntax node.
    pub fn into_syntax(self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node)
    }

    /// Turn the parse into a DOM tree.
    ///
    /// Any semantic errors that occur will be collected
    /// in the returned DOM node.
    pub fn into_dom(self) -> dom::RootNode {
        dom::RootNode::cast(rowan::NodeOrToken::Node(self.into_syntax())).unwrap()
    }
}