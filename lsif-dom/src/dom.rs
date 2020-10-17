use crate::{
    syntax::{SyntaxElement, SyntaxKind, SyntaxKind::*, SyntaxNode, SyntaxToken},
    util::{unescape, StringExt},
};
use indexmap::IndexMap;
use rowan::{TextRange, TextSize};
use std::{hash::Hash, iter::FromIterator, mem, rc::Rc};

#[macro_use]
mod macros;

/// Casting allows constructing DOM nodes from syntax nodes.
pub trait Cast: Sized + private::Sealed {
    fn cast(element: SyntaxElement) -> Option<Self>;
}

pub trait Common: core::fmt::Display + core::fmt::Debug + private::Sealed {
    fn syntax(&self) -> SyntaxElement;
    fn text_range(&self) -> TextRange;

    fn is_valid(&self) -> bool {
        true
    }
}

mod private {
    use super::*;

    pub trait Sealed {}

    dom_sealed!(
        Node,
        RootNode,
        VertexNode,
        // EdgeNode,
        // EntryNode,
        // KeyNode,
        // ValueNode,
        // ArrayNode,
        // IntegerNode,
        // StringNode,
        // BoolNode,
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Root(RootNode),
    Vertex(VertexNode),
    Entry(EntryNode),
}

dom_node_from!(
    RootNode => Root,
    VertexNode => Vertex,
    EntryNode => Entry,
);

impl core::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Root(v) => v.fmt(f),
            Node::Vertex(v) => v.fmt(f),
            Node::Entry(v) => v.fmt(f),
        }
    }
}

impl Common for Node {
    fn syntax(&self) -> SyntaxElement {
        match self {
            Node::Root(v) => v.syntax(),
            Node::Vertex(v) => v.syntax(),
            Node::Entry(v) => v.syntax(),
        }
    }

    fn text_range(&self) -> TextRange {
        match self {
            Node::Root(v) => v.text_range(),
            Node::Vertex(v) => v.text_range(),
            Node::Entry(v) => v.text_ragne()
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            Node::Root(v) => v.is_valid(),
            Node::Vertex(v) => v.is_valid(),
            Node::Entry(v) => v.is_valid(),
        }
    }
}

impl Cast for Node {
    fn cast(element: SyntaxElement) -> Option<Self> {
        match element.kind() {
            // STRING
            // | INTEGER
            // | FLOAT
//             | BOOL
//             => ValueNode::dom_inner(element).map(Node::Value),
//             KEY => KeyNode::cast(element).map(Node::Key),
//             VALUE => ValueNode::cast(element).map(Node::Value),
//             TABLE_HEADER | TABLE_ARRAY_HEADER => TableNode::cast(element).map(Node::Table),
            ENTRY => EntryNode::cast(element).map(Node::Entry),
            VERTEX => EntryNode::cast(element).map(Node::Vertex),
//             ARRAY => ArrayNode::cast(element).map(Node::Array),
//             ROOT => RootNode::cast(element).map(Node::Root),
            _ => None,
        }
    }
}

impl Node {
    pub fn text_range(&self) -> TextRange {
        match self {
            Node::Root(v) => v.text_range(),
            Node::Vertex(v) => v.text_range(),
            // Node::Table(v) => v.text_range(),
            Node::Entry(v) => v.text_range(),
            // Node::Key(v) => v.text_range(),
            // Node::Value(v) => v.text_range(),
            // Node::Array(v) => v.text_range(),
        }
    }

    pub fn kind(&self) -> SyntaxKind {
        match self {
            Node::Root(v) => v.syntax().kind(),
            Node::Vertex(v) => v.syntax().kind(),
            // Node::Table(v) => v.syntax().kind(),
            Node::Entry(v) => v.syntax().kind(),
            // Node::Key(v) => v.syntax().kind(),
            // Node::Value(v) => v.syntax().kind(),
            // Node::Array(v) => v.syntax().kind(),
        }
    }
}

dom_display!(
    RootNode,
    VertexNode,
    // TableNode,
    EntryNode,
    // ArrayNode,
    // IntegerNode,
    // StringNode
);

/// The root of the DOM.
///
/// If any errors occur, the tree might be
/// missing entries, or will be completely empty.
///
/// Syntax errors are **not** reported, those have to
/// be checked before constructing the DOM.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RootNode {
    syntax: SyntaxNode,
    errors: Vec<Error>,
    entries: Entries,
}

impl RootNode {
    pub fn entries(&self) -> &Entries {
        &self.entries
    }

    pub fn into_entries(self) -> Entries {
        self.entries
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
}

impl Common for RootNode {
    fn syntax(&self) -> SyntaxElement {
        self.syntax.clone().into()
    }

    fn text_range(&self) -> TextRange {
        self.syntax.text_range()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Error {
    DuplicatedId { first: KeyNode, second: KeyNode },
    ExpectedTableArray { target: KeyNode, key: KeyNode },
    ExpectedTable { target: KeyNode, key: KeyNode },
    InlineTable { target: KeyNode, key: KeyNode },
    Spanned { range: TextRange, message: String },
    Generic(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DuplicatedId { first, second } => write!(
                f,
                "duplicate keys: \"{}\" ({:?}) and \"{}\" ({:?})",
                &first.full_key_string(),
                &first.text_range(),
                &second.full_key_string(),
                &second.text_range()
            ),
            Error::ExpectedTable { target, key } => write!(
                f,
                "Expected \"{}\" ({:?}) to be a table, but it is not, required by \"{}\" ({:?})",
                &target.full_key_string(),
                &target.text_range(),
                &key.full_key_string(),
                &key.text_range()
            ),
            Error::ExpectedTableArray { target, key } => write!(
                f,
                "\"{}\" ({:?}) conflicts with array of tables: \"{}\" ({:?})",
                &target.full_key_string(),
                &target.text_range(),
                &key.full_key_string(),
                &key.text_range()
            ),
            Error::InlineTable { target, key } => write!(
                f,
                "inline tables cannot be modified: \"{}\" ({:?}), modification attempted here: \"{}\" ({:?})",
                &target.full_key_string(),
                &target.text_range(),
                &key.full_key_string(),
                &key.text_range()
            ),
            Error::Spanned { range, message } => write!(f, "{} ({:?})", message, range),
            Error::Generic(s) => s.fmt(f),
        }
    }
}
impl std::error::Error for Error {}
