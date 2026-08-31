//! `CALL` clauses and `CALL { ... }` subqueries.

use crate::expr::LpgExpr;
use crate::span::Span;
use crate::statement::LpgStatement;

/// A `CALL <procedure>(args) YIELD <items>` invocation of a
/// registered procedure.
///
/// The dotted procedure name (e.g. `apoc.coll.range`) is opaque to
/// the AST; the consumer resolves it against its own procedure
/// registry when translating. Each yield item names a column
/// produced by the procedure (positionally aligned with the
/// procedure's declared outputs) and may be aliased with `AS`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallClause {
    /// The dotted procedure name as written in the source, e.g.
    /// `apoc.coll.range`.
    pub procedure: String,
    /// The argument expressions.
    pub args: Vec<LpgExpr>,
    /// The yield items (each names an output column, optionally
    /// aliased).
    pub yield_items: Vec<YieldItem>,
    /// Source span.
    pub span: Span,
}

/// A `CALL { <query-body> }` subquery.
///
/// The body is a full nested [`LpgStatement`] with its own MATCH /
/// WITH / RETURN. When `import_vars` is empty the subquery is
/// **uncorrelated** and the inner body is evaluated once,
/// independent of the outer; the result is joined (Cartesian) with
/// the outer running output. When `import_vars` is non-empty the
/// subquery is **correlated** — the inner body re-runs per outer
/// row with the listed variables seeded into the inner scope (a
/// lateral join in the consumer's algebra).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallSubquery {
    /// The nested query body (with any leading importing `WITH`
    /// stripped out — its variables are captured in `import_vars`).
    pub body: Box<LpgStatement>,
    /// Outer variables imported by a leading `WITH a, b, ...`
    /// inside the subquery body. Empty for uncorrelated subqueries.
    pub import_vars: Vec<String>,
    /// Source span.
    pub span: Span,
}

/// A single yield item: `name` or `name AS alias`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldItem {
    /// The output column name as named in the YIELD clause.
    pub name: String,
    /// Optional `AS alias` — when present, the bound variable uses
    /// this name instead of `name`.
    pub alias: Option<String>,
    /// Source span.
    pub span: Span,
}
