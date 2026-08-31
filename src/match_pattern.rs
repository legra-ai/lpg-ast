//! `MATCH` clauses and the pattern-element AST (nodes, relationships,
//! quantifiers, path modes).

use crate::binding::PropertyEntry;
use crate::expr::LpgExpr;
use crate::span::Span;

/// A `MATCH` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchClause {
    /// The pattern elements to match.
    pub patterns: Vec<PatternElement>,
    /// Optional `WHERE` filter expression.
    pub where_clause: Option<LpgExpr>,
    /// `true` when the clause was written as `OPTIONAL MATCH`:
    /// non-matching rows pass through with their `MATCH`-introduced
    /// variables left unbound instead of being filtered out (a left
    /// join in the consumer's algebra).
    pub optional: bool,
    /// Source span.
    pub span: Span,
}

/// A pattern element: a node optionally chained with relationships.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternElement {
    /// Optional named-path binding (`p = (a)-[r:T]->(b)`).
    ///
    /// When present, the variable `p` is bound to the alternating
    /// list of node and relationship terms traversed by the pattern.
    pub path_name: Option<String>,
    /// Path-mode marker for `shortestPath(...)` / `allShortestPaths(...)`
    /// wrappers around the pattern. Default is the standard
    /// enumerate-all behaviour.
    pub path_mode: PathMode,
    /// The leading node pattern.
    pub head: NodePattern,
    /// Zero or more `(relationship, node)` pairs forming the chain.
    pub chain: Vec<(RelationshipPattern, NodePattern)>,
    /// Source span.
    pub span: Span,
}

/// Pattern enumeration mode for a [`PatternElement`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PathMode {
    /// Standard enumeration — every matching path is returned.
    #[default]
    Standard,
    /// `shortestPath(...)` — return exactly one path of minimum
    /// hop count per source/destination pair.
    Shortest,
    /// `allShortestPaths(...)` — return every path of minimum hop
    /// count per source/destination pair.
    AllShortest,
}

/// A node pattern `(variable:Label {props})`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodePattern {
    /// Optional variable name.
    pub variable: Option<String>,
    /// Zero or more labels.
    pub labels: Vec<String>,
    /// How the listed labels combine: `AllOf` (`(n:A:B)`, default) or
    /// `AnyOf` (`(n:A|B)`, Neo4j 5+ disjunction).
    pub label_match: LabelMatch,
    /// Inline property entries.
    pub properties: Vec<PropertyEntry>,
    /// Whether an inline property block `{ … }` was written at all —
    /// `true` even for an empty `(n {})`. openCypher treats the presence
    /// of the block (not just non-empty properties) as "with properties"
    /// when deciding `VariableAlreadyBound` on a re-bound CREATE variable
    /// (openCypher TCK, Create1 scenario 19), so an empty block must be
    /// distinguishable from a bare `(n)`.
    pub has_property_block: bool,
    /// Source span.
    pub span: Span,
}

/// How a node pattern's [`labels`](NodePattern::labels) combine.
///
/// The Cypher source separator decides which:
/// - `:A:B` (multi-colon) → [`LabelMatch::AllOf`] (conjunction).
/// - `:A|B` (pipe, Neo4j 5+) → [`LabelMatch::AnyOf`] (disjunction).
///
/// Mixing the two separators in a single node pattern is a parse
/// error. A pattern with zero or one label always lowers to
/// [`LabelMatch::AllOf`] regardless.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LabelMatch {
    /// Every listed label must be present on the matched node.
    #[default]
    AllOf,
    /// At least one listed label must be present on the matched node.
    /// Lowers to a BGP branch per label at translate time.
    AnyOf,
}

/// A relationship pattern `-[variable:TYPE {props}]->`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationshipPattern {
    /// Optional variable name.
    pub variable: Option<String>,
    /// Declared relationship types. Empty = untyped (matches any
    /// relationship); single element = exactly one type; multiple
    /// elements = type alternation `:A|B|C` (matches any of the
    /// listed types). CREATE / MERGE require exactly one type and
    /// will error otherwise.
    pub rel_types: Vec<String>,
    /// Direction of the relationship.
    pub direction: RelDirection,
    /// Optional path quantifier `{min,max}`.
    ///
    /// Present only for GQL queries that include quantified path
    /// patterns. `None` for Cypher patterns.
    pub quantifier: Option<Quantifier>,
    /// Inline edge property entries `-[r:T {key: value, ...}]->`.
    ///
    /// Empty when the relationship has no property block.
    pub properties: Vec<PropertyEntry>,
    /// Source span.
    pub span: Span,
}

/// A relationship variable whose edge value must be reconstructed in a
/// write-then-`RETURN` projection.
///
/// A write statement's trailing `RETURN` is typically translated
/// against a synthetic AST whose `match_clause` is cleared (the
/// prelude MATCH already ran during the write phase), which loses the
/// pattern context a relationship variable needs to render as a full
/// `[:Type {props}]` edge value. This carries the already-materialized
/// endpoint variable names plus type/direction so a translator can
/// rebuild the edge value without re-running the MATCH.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeedRelationship {
    /// The relationship variable (e.g. `r`).
    pub rel_var: String,
    /// The (materialized) source-endpoint node variable name.
    pub from: String,
    /// The (materialized) target-endpoint node variable name.
    pub to: String,
    /// The declared relationship type, when typed (`[r:T]`); `None`
    /// for an untyped `[r]`.
    pub rel_type: Option<String>,
    /// The pattern direction, fixing the subject/object ordering of
    /// the reconstructed edge triple.
    pub direction: RelDirection,
}

/// Direction of a relationship pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelDirection {
    /// `(a)-[r]->(b)` -- left to right.
    Right,
    /// `(a)<-[r]-(b)` -- right to left.
    Left,
    /// `(a)-[r]-(b)` -- undirected / both directions.
    Both,
}

/// A path quantifier specifying repetition bounds.
///
/// Represents `{min,max}` on a relationship pattern (a GQL
/// extension). Cypher uses `*min..max` syntax which maps to the
/// same representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantifier {
    /// Minimum number of hops (inclusive).
    pub min: u64,
    /// Maximum number of hops (inclusive).
    pub max: u64,
}
