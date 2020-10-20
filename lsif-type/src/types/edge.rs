
#[derive(Debug, PartialEq)]
pub enum Item {
    Definition(EdgeData),
    Reference(EdgeData),
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
