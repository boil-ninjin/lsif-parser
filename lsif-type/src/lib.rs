extern crate lsp_types as lsp;

// Use LSP Types
pub type Uri = lsp::Url;
pub type LspRange = lsp::Range;
pub type SymbolKind = lsp::SymbolKind;

// Data Structure types ============================================================================

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub id: u64,
}

#[derive(Debug, PartialEq)]
pub enum Element {
    Vertex(Vertex),
    Edge(Edge),
}

macro_rules! result_of {
    ($x: tt) => {
        <lsp::lsp_request!($x) as lsp::request::Request>::Result
    }
}
#[derive(Debugm PartialEq)]
pub enum Vertex {
    MetaData(MetaData),
    Event(Event),
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#the-project-vertex
    Project(Project),
    // Group,
    Location(Location),
    Document(Document),
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#ranges
    Range(Range),
    // Moniker,
    // PackageInformation,
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#result-set
    ResultSet(ResultSet),
    DocumentSymbolResult(result_of!("textDocument/documentSymbol")),
    FoldingRangeResult(result_of!("textDocument/foldingRange")),
    DocumentLinkResult(result_of!("textDocument/documentLink")),
    DiagnosticResult,
    DeclarationResult,
    // Method results
    DefinitionResult { result: DefinitionResultType },
    // Blocked on https://github.com/gluon-lang/languageserver-types/pull/86
    TypeDefinitionResult(result_of!("textDocument/typeDefinition")),
    HoverResult(result_of!("textDocument/hover")),
    // TODO: Fix ones below to use the { result: LSIFType } format
    ReferenceResult(result_of!("textDocument/references")),
    // Blocked on https://github.com/gluon-lang/languageserver-types/pull/86
    ImplementationResult(result_of!("textDocument/implementation")),
}

pub enum EventKind {
    Begin,
    End,
}

pub enum EventScope {
    Group,
    Project,
    Document,
    MonikerAttach,
}

#[derive(Debug, PartialEq)]
pub struct Event {
    scope: EventKind,
    kind: EventKind,
    data: Element,
}

/// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#resultset
#[derive(Debug, PartialEq)]
pub struct ResultSet {}

#[derive(Debug, PartialEq)]
pub enum Range {
    Declaration(DeclarationRange),
    Definition(DefinitionRange),
    Reference(ReferenceRange),
    Unknown(UnknownRange),
}

#[derive(Debug, PartialEq)]
pub struct DeclarationRange {
    text: String,
    kind: SymbolKind,
    deprecated: Option<boolean>,
    full_range: LspRange,
    detail: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct DefinitionRange {
    text: String,
    kind: SymbolKind,
    deprecated: Option<boolean>,
    full_range: LspRange,
    detail: Option<String>,
}

pub struct ReferenceRange {
    text: String,
}

pub struct UnknownRange {
    text: String,
}

pub type Location = LspRange;

#[derive(Debug, PartialEq)]
pub struct MetaData {
    project_root: Uri,
    position_encoding: String,
    tool_info: Option<MetaDataToolInfo>,
}

#[derive(Debug, PartialEq)]
pub struct MetaDataToolInfo {
    name: String,
    version: Option<String>,
    args: Option<Vec<String>>,
}

/// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#the-project-vertex
#[derive(Debug, PartialEq)]
pub struct Project {
    project_file: Uri,
    language_id: Language,
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
    uri: Uri,
    language_id: Language,
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

/// https://github.com/Microsoft/language-server-protocol/issues/213
/// For examples, see: https://code.visualstudio.com/docs/languages/identifiers.
pub type Language = String;
