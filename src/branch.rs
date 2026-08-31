//! Branch scope for `USE BRANCH "name" { ... }` — an extension for
//! branch-versioned graph stores.

use crate::span::Span;

/// A `USE BRANCH "name" { ... }` scope wrapping the entire statement.
///
/// An extension to Cypher and GQL for graph stores with git-style
/// branching: the wrapped statement is evaluated against the named
/// branch instead of the default branch. Restricted to literal branch
/// names (no parameters or IRIs). The body inside the braces is parsed
/// exactly like a top-level statement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchScope {
    /// The literal branch name (without surrounding quotes).
    pub name: String,
    /// Source span of the entire `USE BRANCH "name" { ... }` clause.
    pub span: Span,
}
