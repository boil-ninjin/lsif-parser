extern crate lsp_types as lsp;


use types::{vertex, edge};


// Use LSP Types ===================================================================================
pub type Uri = lsp::Url;
pub type LspRange = lsp::Range;
pub type SymbolKind = lsp::SymbolKind;
pub type DocumentSymbol = lsp::DocumentSymbol;
pub type Diagnostic = lsp::Diagnostic;
pub type FoldingRange = lsp::FoldingRange;
pub type DocumentLink = lsp::DocumentLink;
pub type Hover = lsp::Hover;

// macros ==========================================================================================
macro_rules! result_of {
    ($x: tt) => {
        <lsp::lsp_request!($x) as lsp::request::Request>::Result
    }
}

macro_rules! dom_display {
    ($($ast: ident),*) => {
        $(
            impl core::fmt::Display for $ast {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                   self.syntax().fmt(f)
                }
            }
        )*
    };
}

macro_rules! add_element_trait_to_vertex {
    ($($ast: ident => $label:tt),*)=> {
        $(
            impl ElementTrait for $ast {
                fn get_type(&self) -> &str {
                    "vertex"
                }
                fn get_label(&self) -> &str{
                    $label
                }
            }

            // impl core::fmt::Display for $ast {
            //     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            //         self.fmt(f)
            //     }
            // }
        )*
    };
}

// Data Structure types ============================================================================
// Common ------------------------------------------------------------------------------------------
/// Repository like git
#[derive(Debug, PartialEq)]
pub struct Repository {
    // avoid to use 'type' for reservation by rust
    /// kind of repo like `git`
    repo_type: String,
    url: String,
    commit_id: Option<String>,
}

// Each types --------------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub struct Entry {
    pub id: u64,
    pub element: Element,
}

#[derive(Debug, PartialEq)]
pub enum Element {
    Vertex(Box<Vertex>),
    Edge(Box<edge::Edge>),
}

pub trait ElementTrait {
    fn get_type(&self) -> &str;
    fn get_label(&self) -> &str;
}

#[derive(Debug, PartialEq)]
pub enum Vertex {
    MetaData(MetaData),
    Event(Event),
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#the-project-vertex
    Project(Project),
    Group(Group),
    // Location(Location),
    Document(Document),
    Moniker(Moniker),
    PackageInformation(PackageInformation),
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#result-set
    ResultSet(ResultSet),
    /// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#ranges
    Range(Range),
    // Method results
    DocumentSymbolResult(DocumentSymbolResultSet),
    FoldingRangeResult(FoldingRangeResult),
    DocumentLinkResult(DocumentLinkResult),
    DiagnosticResult(DiagnosticResult),
    DefinitionResult(DefinitionResult),
    DeclarationResult(DeclarationResult),
    TypeDefinitionResult(TypeDefinitionResult),
    HoverResult(HoverResult),
    ReferenceResult(ReferenceResult),
    ImplementationResult(ImplementationResult),
}

add_element_trait_to_vertex!(
    MetaData=>"metaData",
    Event=>"$event",
    Project=>"project",
	Group=>"group",
	Range=>"range",
	Location=>"location",
	Document=>"document",
	Moniker=>"moniker",
	PackageInformation=>"packageInformation",
	ResultSet=>"resultSet",
    DocumentSymbolResult=>"documentSymbolResult",
    FoldingRangeResult=>"foldingRangeResult",
    DocumentLinkResult=>"documentLinkResult",
    DiagnosticResult=>"diagnosticResult",
    DefinitionResult=>"definitionResult",
    DeclarationResult=>"declarationResult",
    TypeDefinitionResult=>"typeDefinitionResult",
    HoverResult=>"hoverResult",
    ReferenceResult=>"referenceResult",
    ImplementationResult=>"implementationResult",
);

#[derive(Debug, PartialEq)]
pub enum EventKind {
    Begin,
    End,
}

#[derive(Debug, PartialEq)]
pub enum EventScope {
    Group,
    Project,
    Document,
    MonikerAttach,
}

#[derive(Debug, PartialEq)]
pub struct Event {
    scope: EventScope,
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
    deprecated: Option<bool>,
    full_range: LspRange,
    detail: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct DefinitionRange {
    text: String,
    kind: SymbolKind,
    deprecated: Option<bool>,
    full_range: LspRange,
    detail: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct ReferenceRange {
    text: String,
}

#[derive(Debug, PartialEq)]
pub struct UnknownRange {
    text: String,
}

/**
 * A location emittable in LSIF. It has no uri since
 * like ranges locations should be connected to a document
 * using a `contains`edge.
 */
pub type Location = LspRange;

#[derive(Debug, PartialEq)]
pub struct MetaData {
    version: String,
    position_encoding: String,
    tool_info: Option<MetaDataToolInfo>,
}

#[derive(Debug, PartialEq)]
pub struct MetaDataToolInfo {
    name: String,
    version: Option<String>,
    args: Option<Vec<String>>,
}

#[derive(Debug, PartialEq)]
pub struct Group {
    uri: Uri,
    conflict_resolution: GroupConflictResolution,
    name: String,
    root_uri: Uri,
    description: String,
    repository: Option<Repository>,
}

#[derive(Debug, PartialEq)]
pub enum GroupConflictResolution {
    TakeDump,
    TakeDB,
}

/// https://github.com/Microsoft/language-server-protocol/blob/master/indexFormat/specification.md#the-project-vertex
#[derive(Debug, PartialEq)]
pub struct Project {
    kind: LanguageId,
    name: String,
    resource: Option<Uri>,
    contents: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Document {
    uri: Uri,
    language_id: LanguageId,
    contents: Option<String>,
}

/// common type for language id
pub type LanguageId = String;

#[derive(Debug, PartialEq)]
pub enum MonikerKind {
    Import,
    Export,
    Local,
}

#[derive(Debug, PartialEq)]
pub enum UniquenessLevel {
    Document,
    Project,
    Group,
    Schema,
    Global,
}

#[derive(Debug, PartialEq)]
pub struct Moniker {
    schema: String,
    identifier: String,
    unique: UniquenessLevel,
    kind: Option<MonikerKind>,
}

#[derive(Debug, PartialEq)]
pub struct PackageInformation {
    name: String,
    manager: String,
    uri: Option<Uri>,
    content: Option<String>,
    version: Option<String>,
    repository: Option<Repository>,
}

/**
 * A range based document symbol. This allows to reuse already
 * emitted ranges with a `declaration` tag in a document symbol
 * result.
 */
#[derive(Debug, PartialEq)]
pub struct DocumentSymbolRangeResult {
    id: u64,
    pub children: Option<Vec<DocumentSymbolRangeResult>>,
}

#[derive(Debug, PartialEq)]
pub enum DocumentSymbolResultSet {
    DocumentSymbol(DocumentSymbol),
    DocumentSymbolRange(DocumentSymbolRangeResult),
}

pub struct DiagnosticResult {
    result: Vec<Diagnostic>
}

pub struct FoldingRangeResult {
    result: Vec<FoldingRange>
}

pub struct DocumentLinkResult {
    result: Vec<DocumentLink>
}

pub struct DeclarationResult {}

pub struct DefinitionResult {}

pub struct TypeDefinitionResult {}

pub struct ReferenceResult {}

pub struct ImplementationResult {}

pub struct HoverResult {
    result: Hover
}


/// https://github.com/Microsoft/language-server-protocol/issues/213
/// For examples, see: https://code.visualstudio.com/docs/languages/identifiers.
pub type Language = String;
