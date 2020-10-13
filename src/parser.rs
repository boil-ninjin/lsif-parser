//! TOML document to syntax tree parsing.

use crate::{
    dom,
    syntax::{SyntaxKind, SyntaxKind::*, SyntaxNode},
    util::{allowed_chars, check_escape},
};
use dom::Cast;
use logos::{Lexer, Logos};
use rowan::{GreenNode, GreenNodeBuilder, SmolStr, TextRange, TextSize};
use std::convert::TryInto;

#[macro_use]
mod macros;

