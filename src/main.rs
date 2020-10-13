

/// GreenNode is an immutable tree, which is cheap to change,
/// but doesn't contain offsets and parent pointers.
/// You can construct GreenNodes by hand, but a builder
/// is helpful for top-down parsers: it maintains a stack
/// of currently in-progress nodes
use rowan::{GreenNode, GreenNodeBuilder, SmolStr, TextRange, TextSize};
use std::convert::TryInto;
use lsif_parser::syntax::{SyntaxKind, SyntaxKind::*, SyntaxNode};

/// The parse results are stored as a "green tree".
/// We'll discuss working with the results later
#[derive(Debug, Clone)]
struct Parse {
    green_node: GreenNode,
    #[allow(unused)]
    errors: Vec<String>,
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

fn parse(text: &str) -> Parse {
    struct Parser {
        /// input tokens, including whitespace,
        /// in *reverse* order.
        tokens: Vec<(SyntaxKind, SmolStr)>,
        /// the in-progress tree.
        builder: GreenNodeBuilder<'static>,
        /// the list of syntax errors we've accumulated
        /// so far.
        errors: Vec<String>,
    }
    /// The outcome of parsing a single S-expression
    enum SexpRes {
        /// An S-expression (i.e. an atom, or a list) was successfully parsed
        Ok,
        /// Nothing was parsed, as no significant tokens remained
        Eof,
        /// An unexpected ')', ']' ':' was found
        Invalid
    }
    impl Parser {
        fn parse(mut self) -> Parse {
            // Make sure that the root node covers all source
            self.builder.start_node(ROOT.into());
            // Parse zero or more S-expressions
            loop {
                match self.sexp() {
                    SexpRes::Eof => break,
                    SexpRes::Invalid=> {
                        self.builder.start_node(ERROR.into());
                        self.errors.push("unmatched character".to_string());
                        self.bump(); // be sure to chug along in case of error
                        self.builder.finish_node();
                    }
                    SexpRes::Ok => (),
                }
            }
            // Don't forget to eat *trailing* whitespace
            self.skip_ws();
            // Close the root node.
            self.builder.finish_node();

            // Turn the builder into a GreenNode
            Parse { green_node: self.builder.finish(), errors: self.errors }
        }
        fn list(&mut self) {
            assert_eq!(self.current(), Some(L_CURL));
            // Start the list node
            self.builder.start_node(LIST.into());
            self.bump(); // '{'
            loop {
                match self.sexp() {
                    SexpRes::Eof => {
                        self.errors.push("expected `}`".to_string());
                        break;
                    }
                    SexpRes::Invalid=> {
                        self.bump();
                        break;
                    }
                    SexpRes::Ok => (),
                }
            }
            // close the list node
            self.builder.finish_node();
        }
        fn sexp(&mut self) -> SexpRes {
            // Eat leading whitespace
            self.skip_ws();
            // Either a list, an atom, a closing paren,
            // or an eof.
            let t = match self.current() {
                None => return SexpRes::Eof,
                Some(R_CURL) => return SexpRes::Invalid,
                Some(t) => t,
            };
            match t {
                L_CURL => self.list(),
                WORD => {
                    self.builder.start_node(ATOM.into());
                    self.bump();
                    self.builder.finish_node();
                }
                ERROR => self.bump(),
                _ => unreachable!(),
            }
            SexpRes::Ok
        }
        /// Advance one token, adding it to the current branch of the tree builder.
        fn bump(&mut self) {
            let (kind, text) = self.tokens.pop().unwrap();
            self.builder.token(kind.into(), text);
        }
        /// Peek at the first unprocessed token
        fn current(&self) -> Option<SyntaxKind> {
            self.tokens.last().map(|(kind, _)| *kind)
        }
        fn skip_ws(&mut self) {
            while self.current() == Some(WHITESPACE) {
                self.bump()
            }
        }
    }
    let mut tokens = lex(text);
    tokens.reverse();
    Parser { tokens, builder: GreenNodeBuilder::new(), errors: Vec::new() }.parse()
}

/// To work with the parse results we need a view into the
/// green tree - the Syntax tree.
/// It is also immutable, like a GreenNode,
/// but it contains parent pointers, offsets, and
/// has identity semantics.

impl Parse {
    fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

/// Let's check that the parser works as expected
#[test]
fn test_parser() {
    let text = "{ id: 1, type: \"vertex\", label: \"document\", uri: \"file:///Users/dirkb/sample.ts\", languageId: \"typescript\" }";
    let node = parse(text).syntax();
    assert_eq!(
        format!("{:?}", node),
        "ROOT@0..15", // root node, spanning 15 bytes
    );
    assert_eq!(node.children().count(), 1);
    let list = node.children().next().unwrap();
    let children = list
        .children_with_tokens()
        .map(|child| format!("{:?}@{:?}", child.kind(), child.text_range()))
        .collect::<Vec<_>>();

    assert_eq!(
        children,
        vec![
            "L_CURL@0..1".to_string(),
            "ATOM@1..2".to_string(),
            "WHITESPACE@2..3".to_string(), // note, explicit whitespace!
            "LIST@3..11".to_string(),
            "WHITESPACE@11..12".to_string(),
            "ATOM@12..14".to_string(),
            "R_CURL@14..15".to_string(),
        ]
    );
}

/// So far, we've been working with a homogeneous untyped tree.
/// It's nice to provide generic tree operations, like traversals,
/// but it's a bad fit for semantic analysis.
/// This crate itself does not provide AST facilities directly,
/// but it is possible to layer AST on top of `SyntaxNode` API.
/// Let's write a function to evaluate S-expression.
///
/// For that, let's define AST nodes.
/// It'll be quite a bunch of repetitive code, so we'll use a macro.
///
/// For a real language, you'd want to generate an AST. I find a
/// combination of `serde`, `ron` and `tera` crates invaluable for that!
macro_rules! ast_node {
    ($ast:ident, $kind:ident) => {
        #[derive(PartialEq, Eq, Hash)]
        #[repr(transparent)]
        struct $ast(SyntaxNode);
        impl $ast {
            #[allow(unused)]
            fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == $kind {
                    Some(Self(node))
                } else {
                    None
                }
            }
        }
    };
}

ast_node!(Root, ROOT);
ast_node!(Atom, ATOM);
ast_node!(List, LIST);

// Sexp is slightly different, so let's do it by hand.
#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
struct Sexp(SyntaxNode);

enum SexpKind {
    Atom(Atom),
    List(List),
}

impl Sexp {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if Atom::cast(node.clone()).is_some() || List::cast(node.clone()).is_some() {
            Some(Sexp(node))
        } else {
            None
        }
    }

    fn kind(&self) -> SexpKind {
        Atom::cast(self.0.clone())
            .map(SexpKind::Atom)
            .or_else(|| List::cast(self.0.clone()).map(SexpKind::List))
            .unwrap()
    }
}

// Let's enhance AST nodes with ancillary functions and
// eval.
impl Root {
    fn sexps(&self) -> impl Iterator<Item = Sexp> + '_ {
        self.0.children().filter_map(Sexp::cast)
    }
}

enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Atom {
    fn eval(&self) -> Option<i64> {
        self.text().parse().ok()
    }
    fn as_op(&self) -> Option<Op> {
        let op = match self.text().as_str() {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => return None,
        };
        Some(op)
    }
    fn text(&self) -> &SmolStr {
        match &self.0.green().children().next() {
            Some(rowan::NodeOrToken::Token(token)) => token.text(),
            _ => unreachable!(),
        }
    }
}

impl List {
    fn sexps(&self) -> impl Iterator<Item = Sexp> + '_ {
        self.0.children().filter_map(Sexp::cast)
    }
    fn eval(&self) -> Option<i64> {
        let op = match self.sexps().nth(0)?.kind() {
            SexpKind::Atom(atom) => atom.as_op()?,
            _ => return None,
        };
        let arg1 = self.sexps().nth(1)?.eval()?;
        let arg2 = self.sexps().nth(2)?.eval()?;
        let res = match op {
            Op::Add => arg1 + arg2,
            Op::Sub => arg1 - arg2,
            Op::Mul => arg1 * arg2,
            Op::Div if arg2 == 0 => return None,
            Op::Div => arg1 / arg2,
        };
        Some(res)
    }
}

impl Sexp {
    fn eval(&self) -> Option<i64> {
        match self.kind() {
            SexpKind::Atom(atom) => atom.eval(),
            SexpKind::List(list) => list.eval(),
        }
    }
}

impl Parse {
    fn root(&self) -> Root {
        Root::cast(self.syntax()).unwrap()
    }
}

/// Let's test the eval!
fn main() {
    let sexps = "
92
(+ 62 30)
(/ 92 0)
nan
(+ (* 15 2) 62)
";
    let root = parse(sexps).root();
    let res = root.sexps().map(|it| it.eval()).collect::<Vec<_>>();
    eprintln!("{:?}", res);
    assert_eq!(res, vec![Some(92), Some(92), None, None, Some(92),])
}

/// Split the input string into a flat list of tokens
/// (such as L_CURL, WORD, and WHITESPACE)
fn lex(text: &str) -> Vec<(SyntaxKind, SmolStr)> {
    fn tok(t: SyntaxKind) -> m_lexer::TokenKind {
        m_lexer::TokenKind(rowan::SyntaxKind::from(t).0)
    }
    fn kind(t: m_lexer::TokenKind) -> SyntaxKind {
        match t.0 {
            0 => L_CURL,
            1 => R_CURL,
            2 => WORD,
            3 => WHITESPACE,
            4 => ERROR,
            _ => unreachable!(),
        }
    }

    let lexer = m_lexer::LexerBuilder::new()
        .error_token(tok(ERROR))
        .tokens(&[
            (tok(L_CURL), r"\("),
            (tok(R_CURL), r"\)"),
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
