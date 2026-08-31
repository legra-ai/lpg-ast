//! `RETURN` clause and its associated projection / ORDER BY items.

use crate::expr::LpgExpr;
use crate::span::Span;

/// A `RETURN` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnClause {
    /// Whether `DISTINCT` was specified.
    pub distinct: bool,
    /// Whether `*` was specified (`RETURN *`).
    pub star: bool,
    /// The projected items.
    pub items: Vec<ReturnItem>,
    /// GQL `GROUP BY` expressions (may be empty).
    pub group_by: Vec<LpgExpr>,
    /// GQL `HAVING` filter expression (applied after grouping).
    pub having: Option<LpgExpr>,
    /// Optional `ORDER BY` items.
    pub order_by: Vec<OrderItem>,
    /// Optional `SKIP` count.
    pub skip: Option<u64>,
    /// Optional `LIMIT` count.
    pub limit: Option<u64>,
    /// Source span.
    pub span: Span,
}

/// A single item in a RETURN clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnItem {
    /// The expression to return.
    pub expr: LpgExpr,
    /// Original source text for the expression.
    pub source: String,
    /// Optional alias (`AS name`).
    pub alias: Option<String>,
    /// Source span.
    pub span: Span,
}

/// A single item in an ORDER BY clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderItem {
    /// The expression to sort by.
    pub expr: LpgExpr,
    /// Whether to sort ascending (true) or descending (false).
    pub ascending: bool,
    /// Source span.
    pub span: Span,
}
