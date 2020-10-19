extern crate lsp_types as lsp;

pub use lsp::Url;
pub use lsp::{NumberOrString, Range, Position};

pub type RangeId = lsp::NumberOrString;

#[derive(Debug, PartialEq)]
pub enum LocationOrRangeId {
    Location(lsp::Location),
    RangeId(RangeId),
}

macro_rules! result_of {
    ($x: tt) => {
        <lsp::lsp_request!($x) as lsp::request::Request>::Result
    }
}

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub id: lsp::NumberOrString,
    data: Element,
}

#[derive(Debug, PartialEq)]
pub enum Element {
    Vertex(Vertex),
    Edge(Edge),
}

#[derive(Debug, PartialEq)]
pub enum Vertex {
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#the-project-vertex
    Project(Project),
    Document(Document),
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#ranges
    Range(lsp::Range),
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#result-set
    ResultSet(ResultSet),

    // Method results
    DefinitionResult { result: DefinitionResultType },
    // TODO: Fix ones below to use the { result: LSIFType } format
    HoverResult(result_of!("textDocument/hover")),
    ReferenceResult(result_of!("textDocument/references")),
    // Blocked on https://github.com/gluon-lang/languageserver-types/pull/86
    // ImplementationResult(result_of!("textDocument/implementation")),
    // Blocked on https://github.com/gluon-lang/languageserver-types/pull/86
    // TypeDefinitionResult(result_of!("textDocument/typeDefinition")),
    FoldingRangeResult(result_of!("textDocument/foldingRange")),
    DocumentLinkResult(result_of!("textDocument/documentLink")),
    DocumentSymbolResult(result_of!("textDocument/documentSymbol")),
    // TODO (these below and more)
    DiagnosticResult,
    ExportResult,
    ExternalImportResult,
}

#[derive(Debug, PartialEq)]
pub enum Edge {
    Contains(EdgeData),
    RefersTo(EdgeData),
    Item(Item),

    // Methods
    Definition(EdgeData),
    Declaration(EdgeData),
    Hover(EdgeData),
    References(EdgeData),
    Implementation(EdgeData),
    TypeDefinition(EdgeData),
    FoldingRange(EdgeData),
    DocumentLink(EdgeData),
    DocumentSymbol(EdgeData),
    Diagnostic(EdgeData),
}

#[derive(Debug, PartialEq)]
pub struct EdgeData {
    in_v: lsp::NumberOrString,
    out_v: lsp::NumberOrString,
}

#[derive(Debug, PartialEq)]
pub enum DefinitionResultType {
    Scalar(LocationOrRangeId),
    Array(LocationOrRangeId),
}

#[derive(Debug, PartialEq)]
pub enum Item {
    Definition(EdgeData),
    Reference(EdgeData),
}

#[derive(Debug, PartialEq)]
pub struct Document {
    uri: lsp::Url,
    language_id: Language,
}

/// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#result-set
#[derive(Debug, PartialEq)]
pub struct ResultSet {
    key: Option<String>,
}

/// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#the-project-vertex
#[derive(Debug, PartialEq)]
pub struct Project {
    project_file: lsp::Url,
    language_id: Language,
}

/// https://github.com/Microsoft/language-server-protocol/issues/213
/// For examples, see: https://code.visualstudio.com/docs/languages/identifiers.
pub type Language = String;
