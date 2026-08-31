//! Top-level LPG statement and union-branch types.

use crate::binding::{
    ForClause,
    LetBinding,
};
use crate::branch::BranchScope;
use crate::call::{
    CallClause,
    CallSubquery,
};
use crate::expr::LpgExpr;
use crate::match_pattern::{
    MatchClause,
    SeedRelationship,
};
use crate::return_clause::ReturnClause;
use crate::span::Span;
use crate::with::WithPart;
use crate::write::LpgWriteClause;

/// A complete LPG statement — either read-only (MATCH / WITH /
/// RETURN) or write-only (CREATE / DELETE / SET / MERGE / REMOVE).
///
/// **Read shape**:
///
/// ```text
/// (CALL ... YIELD ...)* (MATCH ... WITH ...)* MATCH? FILTER? LET? FOR? RETURN ...
/// ```
///
/// **Write shape**:
///
/// ```text
/// CREATE (pattern-elements...) [, CREATE / DELETE / SET / MERGE / REMOVE ...]
/// ```
///
/// When [`write_clauses`](LpgStatement::write_clauses) is non-empty
/// the statement is a write and the other fields are conventionally
/// empty. Consumers route via [`LpgStatement::is_write`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LpgStatement {
    /// Leading `CALL <procedure>(args) YIELD ...` procedure
    /// invocations (may be empty).
    pub call_clauses: Vec<CallClause>,
    /// Leading `MATCH ... WITH ...` parts (may be empty for simple
    /// queries).
    pub with_parts: Vec<WithPart>,
    /// The final MATCH clause (for the RETURN part). `None` when the
    /// last WITH directly feeds into RETURN without an intervening
    /// MATCH.
    pub match_clause: Option<MatchClause>,
    /// GQL `FILTER` expressions applied after MATCH (may be empty).
    pub filter_clauses: Vec<LpgExpr>,
    /// GQL `LET` bindings for intermediate variable assignments (may
    /// be empty).
    pub let_bindings: Vec<LetBinding>,
    /// GQL `FOR` iteration clause (optional).
    pub for_clause: Option<ForClause>,
    /// Nested `CALL { <query-body> }` subqueries that run after the
    /// outer MATCH / WITH stages and before the final RETURN.
    /// Multiple subqueries chain — each runs against the running
    /// output of the prior stage.
    pub call_subqueries: Vec<CallSubquery>,
    /// The terminal RETURN clause with projections and modifiers.
    pub return_clause: ReturnClause,
    /// Optional outer branch scope produced by `USE BRANCH "name"
    /// { ... }` (Cypher) or `USE BRANCH 'name' { ... }` (GQL). When
    /// `Some`, the entire statement is evaluated against the named
    /// branch (see [`BranchScope`]).
    pub branch_scope: Option<BranchScope>,
    /// Write clauses (CREATE / DELETE / SET / MERGE / REMOVE).
    /// Empty for read statements; non-empty for write statements
    /// (in which case the read fields are conventionally empty).
    /// See [`LpgStatement::is_write`].
    pub write_clauses: Vec<LpgWriteClause>,
    /// Additional `UNION [ALL]?`-separated branches that follow
    /// this statement. Empty for single-body queries. When
    /// non-empty, this statement is the first branch of a union
    /// and `union_tail` carries the remaining branches in source
    /// order; each tail entry records whether the immediately
    /// preceding separator was `UNION ALL` (no de-duplication) or
    /// `UNION` (distinct).
    pub union_tail: Vec<UnionBranch>,
    /// Relationship variables to reconstruct as edge values in a
    /// write-then-`RETURN` projection. Empty for parsed statements;
    /// a translator may fill it on a synthetic RETURN AST so a
    /// relationship variable renders as an edge value instead of a
    /// bare edge identifier. See [`SeedRelationship`].
    pub seed_relationships: Vec<SeedRelationship>,
    /// Source span.
    pub span: Span,
}

/// A single tail branch of a multi-body `UNION` query.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionBranch {
    /// Whether the preceding separator was `UNION ALL` (`true`,
    /// retain duplicates) vs `UNION` (`false`, distinct).
    pub all: bool,
    /// The branch body.
    pub body: LpgStatement,
    /// Source span covering the separator and the body.
    pub span: Span,
}

impl LpgStatement {
    /// Returns `true` when this statement is a write (one or more
    /// CREATE / DELETE / SET / MERGE / REMOVE clauses). Consumers
    /// use this to route between read and update execution paths.
    #[must_use]
    pub fn is_write(&self) -> bool {
        !self.write_clauses.is_empty()
    }
}
