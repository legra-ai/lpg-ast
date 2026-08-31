//! `WITH` clauses and `MATCH ... WITH ...` parts.

use crate::binding::ForClause;
use crate::expr::LpgExpr;
use crate::match_pattern::MatchClause;
use crate::return_clause::{
    OrderItem,
    ReturnItem,
};
use crate::span::Span;

/// A single `MATCH ... WITH ...` part in a multi-part query.
///
/// Each part composes any combination of an optional MATCH and an
/// optional UNWIND (the `for_clause`) that runs before the WITH
/// projection. Both may be `None` for bare `WITH 1 AS x` style
/// pipelines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithPart {
    /// The MATCH clause preceding the WITH. `None` when no MATCH
    /// precedes the WITH within this part.
    pub match_clause: Option<MatchClause>,
    /// An `UNWIND expr AS variable` preceding the WITH. Applied
    /// after `match_clause` (when present) and before the WITH
    /// projection, so `MATCH ... UNWIND ... WITH ...` and
    /// `UNWIND ... WITH ...` both work.
    pub for_clause: Option<ForClause>,
    /// The WITH projection.
    pub with_clause: WithClause,
    /// Source span.
    pub span: Span,
}

/// A `WITH` clause — an intermediate projection that pipes results
/// into the next query part.
///
/// Structurally identical to
/// [`ReturnClause`](crate::return_clause::ReturnClause) with an additional
/// optional WHERE filter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WithClause {
    /// Whether `DISTINCT` was specified.
    pub distinct: bool,
    /// Whether `*` was specified (`WITH *`).
    pub star: bool,
    /// The projected items.
    pub items: Vec<ReturnItem>,
    /// Optional `WHERE` filter (applied after projection).
    pub where_clause: Option<LpgExpr>,
    /// Optional `ORDER BY` items.
    pub order_by: Vec<OrderItem>,
    /// Optional `SKIP` count.
    pub skip: Option<u64>,
    /// Optional `LIMIT` count.
    pub limit: Option<u64>,
    /// Source span.
    pub span: Span,
}
