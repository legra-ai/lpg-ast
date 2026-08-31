//! Write clauses: CREATE / DELETE / SET / MERGE / REMOVE / FOREACH.

use crate::binding::PropertyEntry;
use crate::expr::LpgExpr;
use crate::match_pattern::PatternElement;
use crate::span::Span;

/// A single write clause in an LPG statement.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::large_enum_variant)]
pub enum LpgWriteClause {
    /// `CREATE (pattern-elements...)` — insert nodes and
    /// relationships.
    Create(CreateClause),
    /// `[DETACH] DELETE var (, var)*` — remove one or more bound
    /// graph entities. Requires a preceding MATCH to bind the
    /// targets.
    Delete(DeleteClause),
    /// `SET item (, item)*` — update properties or add labels on
    /// bound entities. Requires a preceding MATCH.
    Set(SetClause),
    /// `REMOVE item (, item)*` — drop properties or labels from
    /// bound entities. Requires a preceding MATCH. The natural
    /// inverse of `SET` for property and label items.
    Remove(RemoveClause),
    /// `MERGE pattern (ON MATCH SET ...)? (ON CREATE SET ...)?` —
    /// match-or-create: if `pattern` already exists, optional
    /// `ON MATCH SET` runs against the existing binding;
    /// otherwise the pattern is inserted and optional
    /// `ON CREATE SET` runs against the new binding.
    Merge(MergeClause),
    /// `FOREACH (var IN list | mutation-clauses)` — iterate `list`
    /// binding each element to `var` and execute the nested mutation
    /// clauses per element.
    Foreach(ForeachClause),
}

/// A `FOREACH (var IN source | body)` clause.
///
/// Property values inside the body may reference `variable`, which is
/// substituted with the current iteration's element. Consumers may
/// restrict which clause kinds the body accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeachClause {
    /// Iteration variable bound to each element of `source` while
    /// the body executes.
    pub variable: String,
    /// The list expression to iterate over.
    pub source: LpgExpr,
    /// The nested mutation clauses executed per iteration.
    pub body: Vec<LpgWriteClause>,
    /// Source span.
    pub span: Span,
}

/// A `MERGE` clause.
///
/// The optional `ON MATCH` / `ON CREATE` SET clauses reuse the
/// regular [`SetClause`] AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeClause {
    /// The pattern to match-or-create.
    pub pattern: PatternElement,
    /// SET items applied when the pattern matched existing data.
    pub on_match: Option<SetClause>,
    /// SET items applied when the pattern was newly created.
    pub on_create: Option<SetClause>,
    /// Source span.
    pub span: Span,
}

/// A `REMOVE` clause: a comma-separated list of [`RemoveItem`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveClause {
    /// The individual remove items.
    pub items: Vec<RemoveItem>,
    /// Source span.
    pub span: Span,
}

/// A single drop step inside a `REMOVE` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoveItem {
    /// `var.key` — drop any current value of the property on the
    /// bound entity. For relationship variables, this drops the edge
    /// property (the edge itself stays).
    Property {
        /// Cypher variable name bound by the preceding MATCH.
        var: String,
        /// Property key.
        key: String,
        /// Source span.
        span: Span,
    },
    /// `var:Label[:Label2...]` — drop one or more labels from the
    /// bound node.
    Label {
        /// Cypher variable name.
        var: String,
        /// Label names to remove.
        labels: Vec<String>,
        /// Source span.
        span: Span,
    },
}

/// A `SET` clause: a comma-separated list of [`SetItem`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetClause {
    /// The individual update items.
    pub items: Vec<SetItem>,
    /// Source span.
    pub span: Span,
}

/// A single update step inside a `SET` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SetItem {
    /// `var.key = value` — replace any existing property value.
    Property {
        /// Cypher variable name bound by the preceding MATCH.
        var: String,
        /// Property key.
        key: String,
        /// New value expression.
        value: LpgExpr,
        /// Source span.
        span: Span,
    },
    /// `var:Label[:Label2...]` — add one or more labels to the
    /// bound node.
    Label {
        /// Cypher variable name.
        var: String,
        /// Label names to add (already de-duplicated by the
        /// parser).
        labels: Vec<String>,
        /// Source span.
        span: Span,
    },
    /// `var += { key: value, ... }` — merge a property map into
    /// the bound node, overwriting keys present in the map while
    /// leaving other properties intact.
    Merge {
        /// Cypher variable name.
        var: String,
        /// Property entries to merge.
        properties: Vec<PropertyEntry>,
        /// Source span.
        span: Span,
    },
    /// `var = { key: value, ... }` — replace every literal-valued
    /// property on the bound node with the map's entries. Labels
    /// and outgoing relationships stay untouched.
    Replace {
        /// Cypher variable name.
        var: String,
        /// Property entries that define the new property set.
        properties: Vec<PropertyEntry>,
        /// Source span.
        span: Span,
    },
    /// `var = otherEntity` — replace every literal-valued property on
    /// the bound target with a copy of `source`'s properties
    /// (openCypher TCK, Merge6 scenario 6 / Merge7 scenario 4:
    /// `SET r = a`). `source` is a bound node or relationship
    /// variable.
    ReplaceFromEntity {
        /// Target Cypher variable name.
        var: String,
        /// Source entity variable whose properties are copied.
        source: String,
        /// Source span.
        span: Span,
    },
}

/// A `[DETACH] DELETE` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteClause {
    /// Variable names to delete (must be bound by the preceding
    /// MATCH).
    pub targets: Vec<String>,
    /// True when the user wrote `DETACH DELETE` — removes incident
    /// edges as well as the node itself.
    pub detach: bool,
    /// Source span.
    pub span: Span,
}

/// A `CREATE` clause.
///
/// Carries one or more pattern elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateClause {
    /// The pattern elements to insert. Each is a node optionally
    /// chained with relationships.
    pub patterns: Vec<PatternElement>,
    /// Source span.
    pub span: Span,
}
