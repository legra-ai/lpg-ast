//! `LET` and `FOR` bindings, plus inline property entries.

use crate::expr::LpgExpr;
use crate::span::Span;

/// A GQL `LET` binding: `variable = expression`.
///
/// Introduces a new variable computed from an expression. Cypher
/// does not use LET; it achieves the same through WITH projections.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetBinding {
    /// The variable name being bound.
    pub variable: String,
    /// The expression computing the value.
    pub expr: LpgExpr,
    /// Source span.
    pub span: Span,
}

/// A GQL `FOR` clause: `FOR variable IN expression`.
///
/// Iterates over a list-valued expression, binding each element to
/// the named variable for downstream clauses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForClause {
    /// The iteration variable name.
    pub variable: String,
    /// The list expression to iterate over.
    pub expr: LpgExpr,
    /// Source span.
    pub span: Span,
}

/// An inline property entry `key: value`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyEntry {
    /// The property key.
    pub key: String,
    /// The property value expression.
    pub value: LpgExpr,
    /// Source span.
    pub span: Span,
}
