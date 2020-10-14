// use crate::{
//     syntax::{SyntaxElement, SyntaxKind, SyntaxKind::*, SyntaxNode, SyntaxToken},
//     util::{unescape, StringExt},
// };
// use indexmap::IndexMap;
// use rowan::{TextRange, TextSize};
// use std::{hash::Hash, iter::FromIterator, mem, rc::Rc};
//
// #[macro_use]
// mod macros;
//
// /// Casting allows constructing DOM nodes from syntax nodes.
// pub trait Cast: Sized + private::Sealed {
//     fn cast(element: SyntaxElement) -> Option<Self>;
// }
//
// pub trait Common: core::fmt::Display + core::fmt::Debug + private::Sealed {
//     fn syntax(&self) -> SyntaxElement;
//     fn text_range(&self) -> TextRange;
//
//     fn is_valid(&self) -> bool {
//         true
//     }
// }
//
// mod private {
//     use super::*;
//
//     pub trait Sealed {}
//     dom_sealed!(
//         Node,
//         RootNode,
//         EntryNode,
//         KeyNode,
//         ValueNode,
//         ArrayNode,
//         IntegerNode,
//         StringNode,
//         BoolNode,
//     );
// }
//
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum Node {
//     Root(RootNode),
//     Entry(EntryNode),
// }
//
// dom_node_from!(
//     RootNode => Root,
//     EntryNode => Entry,
// );
//
// impl core::fmt::Display for Node {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Node::Root(v) => v.fmt(f),
//             Node::Entry(v) => v.fmt(f),
//         }
//     }
// }
//
// impl Common for Node {
//     fn syntax(&self) -> SyntaxElement {
//         match self {
//             Node::Root(v) => v.syntax(),
//             Node::Entry(v) => v.syntax(),
//         }
//     }
//
//     fn text_range(&self) -> TextRange {
//         match self {
//             Node::Root(v) => v.text_range(),
//             Node::Entry(v) => v.text_range(),
//         }
//     }
//
//     fn is_valid(&self) -> bool {
//         match self {
//             Node::Root(v) => v.is_valid(),
//             Node::Entry(v) => v.is_valid(),
//         }
//     }
// }
//
// impl Cast for Node {
//     fn cast(element: SyntaxElement) -> Option<Self> {
//         match element.kind() {
//             STRING
//             | INTEGER
//             | FLOAT
//             | BOOL
//             => ValueNode::dom_inner(element).map(Node::Value),
//             KEY => KeyNode::cast(element).map(Node::Key),
//             VALUE => ValueNode::cast(element).map(Node::Value),
//             TABLE_HEADER | TABLE_ARRAY_HEADER => TableNode::cast(element).map(Node::Table),
//             ENTRY => EntryNode::cast(element).map(Node::Entry),
//             ARRAY => ArrayNode::cast(element).map(Node::Array),
//             ROOT => RootNode::cast(element).map(Node::Root),
//             _ => None,
//         }
//     }
// }