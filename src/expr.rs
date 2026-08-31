//! LPG expression AST and binary/unary operators.

use crate::match_pattern::{
    LabelMatch,
    MatchClause,
};
use crate::span::Span;

/// An LPG expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpgExpr {
    /// A plain identifier (variable reference).
    Identifier {
        /// The identifier name.
        name: String,
        /// Source span.
        span: Span,
    },
    /// A runtime parameter reference (`$name`).
    Parameter {
        /// Parameter name without the leading `$`.
        name: String,
        /// Source span.
        span: Span,
    },
    /// Property access: `expr.property`.
    PropertyAccess {
        /// The object expression.
        object: String,
        /// The property name.
        property: String,
        /// Source span.
        span: Span,
    },
    /// A string literal.
    StringLit {
        /// The string value (without quotes).
        value: String,
        /// Source span.
        span: Span,
    },
    /// An integer literal.
    IntegerLit {
        /// The integer value.
        value: i64,
        /// Source span.
        span: Span,
    },
    /// A floating-point literal.
    FloatLit {
        /// The float value as raw text.
        value: String,
        /// Source span.
        span: Span,
    },
    /// A boolean literal.
    BooleanLit {
        /// The boolean value.
        value: bool,
        /// Source span.
        span: Span,
    },
    /// A `NULL` literal.
    NullLit {
        /// Source span.
        span: Span,
    },
    /// A binary operation.
    BinaryOp {
        /// The operator.
        op: LpgBinaryOp,
        /// Left operand.
        left: Box<LpgExpr>,
        /// Right operand.
        right: Box<LpgExpr>,
        /// Source span.
        span: Span,
    },
    /// A unary operation.
    UnaryOp {
        /// The operator.
        op: LpgUnaryOp,
        /// The operand.
        operand: Box<LpgExpr>,
        /// Source span.
        span: Span,
    },
    /// A function call.
    FunctionCall {
        /// The function name.
        name: String,
        /// Whether `DISTINCT` was specified (for aggregates).
        distinct: bool,
        /// The arguments.
        args: Vec<LpgExpr>,
        /// Source span.
        span: Span,
    },
    /// `expr IS NULL`.
    IsNull {
        /// The expression to test.
        expr: Box<LpgExpr>,
        /// Source span.
        span: Span,
    },
    /// `expr IS NOT NULL`.
    IsNotNull {
        /// The expression to test.
        expr: Box<LpgExpr>,
        /// Source span.
        span: Span,
    },
    /// `expr IS [NOT] :Label` / `IS :A|B` / `IS :A:B` —
    /// Neo4j 5+ label-test predicate. Evaluates to `true` when the
    /// node bound by `target` carries the labels matching
    /// `label_match`. With `negated`, the polarity flips.
    LabelTest {
        /// The expression that should resolve to a graph node.
        target: Box<LpgExpr>,
        /// The labels to test for.
        labels: Vec<String>,
        /// Conjunction (`:A:B`) vs disjunction (`:A|B`) semantics.
        label_match: LabelMatch,
        /// Whether the test was written `IS NOT :Label`.
        negated: bool,
        /// Source span.
        span: Span,
    },
    /// `expr IN [list]`.
    In {
        /// The expression to test.
        expr: Box<LpgExpr>,
        /// The right-hand side. When `dynamic` is `false` this holds
        /// the elements of a static list literal (`x IN [a, b, c]`).
        /// When `dynamic` is `true` it holds exactly one expression —
        /// a list-*valued* expression (`x IN coll`, `x IN $param`,
        /// `x IN fn(...)`) — whose runtime list value is iterated.
        list: Vec<LpgExpr>,
        /// Whether the right-hand side is a single list-valued
        /// expression (`true`) rather than a static list literal
        /// (`false`). See [`LpgExpr::In::list`].
        dynamic: bool,
        /// Source span.
        span: Span,
    },
    /// A list literal `[e1, e2, ...]`.
    ListLit {
        /// The elements.
        elements: Vec<LpgExpr>,
        /// Source span.
        span: Span,
    },
    /// A map literal `{k1: e1, k2: e2, ...}` in expression
    /// position (e.g. `RETURN {name: p.name, age: p.age}`).
    /// Inline-property maps inside patterns and `SET` are not
    /// represented as `MapLit` — those flow through the dedicated
    /// `PropertyMap` lowering paths instead.
    MapLit {
        /// The entries in source order.
        entries: Vec<MapEntry>,
        /// Source span.
        span: Span,
    },
    /// A list comprehension `[var IN source (WHERE filter)? ('|'
    /// projection)?]`. Evaluates by iterating `source`, optionally
    /// filtering by `filter`, optionally projecting by
    /// `projection`, and collecting the results into a list.
    ListComp {
        /// The bound iteration variable.
        variable: String,
        /// The source list expression.
        source: Box<LpgExpr>,
        /// Optional `WHERE` predicate.
        filter: Option<Box<LpgExpr>>,
        /// Optional `|` projection. When `None`, each kept element
        /// is yielded directly.
        projection: Option<Box<LpgExpr>>,
        /// Source span.
        span: Span,
    },
    /// A list predicate — `ANY(var IN source WHERE pred)`,
    /// `ALL(...)`, `NONE(...)`, or `SINGLE(...)`. Evaluates the
    /// predicate per element and folds the results to a boolean
    /// per the predicate kind.
    ListPred {
        /// Which predicate semantics to apply.
        kind: LpgListPredicateKind,
        /// The bound iteration variable.
        variable: String,
        /// The source list expression.
        source: Box<LpgExpr>,
        /// The boolean predicate evaluated with `variable` bound.
        predicate: Box<LpgExpr>,
        /// Source span.
        span: Span,
    },
    /// A `*` expression (used in `RETURN *`).
    Star {
        /// Source span.
        span: Span,
    },
    /// `EXISTS { MATCH ... }` or `NOT EXISTS { MATCH ... }`.
    Exists {
        /// Whether this is `NOT EXISTS`.
        negated: bool,
        /// The subquery match clause.
        subquery: Box<MatchClause>,
        /// Source span.
        span: Span,
    },
    /// A Cypher pattern comprehension
    /// `[pattern_element (WHERE pred)? '|' projection]`. The
    /// `pattern_element` (with optional WHERE) is lowered into a
    /// [`MatchClause`] so it can reuse the standard pattern
    /// translation; the `projection` is evaluated per matching row
    /// and collected into a list.
    PatternComp {
        /// The inline pattern (with optional WHERE filter).
        pattern: Box<MatchClause>,
        /// Per-row projection expression.
        projection: Box<LpgExpr>,
        /// Source span.
        span: Span,
    },
    /// A `COUNT { ... }` subquery expression — Neo4j 5+. Evaluates
    /// the inline pattern (correlated with the surrounding bindings)
    /// and returns the number of matching rows as an integer.
    CountSubquery {
        /// The inline pattern (with optional WHERE filter).
        pattern: Box<MatchClause>,
        /// Source span.
        span: Span,
    },
    /// A Cypher `reduce(acc = init, x IN source | expr)` fold. The
    /// accumulator starts at `init`; for each element of `source`
    /// (bound to `variable`), `expr` is evaluated with both
    /// `accumulator` and `variable` in scope and replaces the
    /// accumulator. The final accumulator value is the result.
    Reduce {
        /// Accumulator variable name (left of `=`).
        accumulator: String,
        /// Initial accumulator value.
        init: Box<LpgExpr>,
        /// Iteration variable name (between `,` and `IN`).
        variable: String,
        /// Source list expression.
        source: Box<LpgExpr>,
        /// Step expression evaluated per element.
        expr: Box<LpgExpr>,
        /// Source span.
        span: Span,
    },
    /// `CASE [input]? WHEN match THEN result … (ELSE default)? END`.
    ///
    /// When `input` is `Some`, each branch matches `input = match`
    /// (simple form). When `input` is `None`, each branch's `match`
    /// is a boolean condition (generic form).
    Case {
        /// The discriminator expression for the simple form, or
        /// `None` for the generic form.
        input: Option<Box<LpgExpr>>,
        /// The `WHEN … THEN …` branches, in source order.
        branches: Vec<CaseBranch>,
        /// The optional `ELSE` default.
        else_expr: Option<Box<LpgExpr>>,
        /// Source span.
        span: Span,
    },
}

/// Which list predicate to apply to a per-element boolean stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LpgListPredicateKind {
    /// `ANY` — at least one element matches.
    Any,
    /// `ALL` — every element matches (vacuously true for empty
    /// sources).
    All,
    /// `NONE` — no element matches.
    None,
    /// `SINGLE` — exactly one element matches.
    Single,
}

/// A single `key: value` entry of a map literal expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapEntry {
    /// The key (always an identifier in Cypher; stored as its
    /// source text without quotes).
    pub key: String,
    /// The associated value expression.
    pub value: LpgExpr,
    /// Source span covering `key: value`.
    pub span: Span,
}

/// A single `WHEN match THEN result` branch of a `CASE` expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseBranch {
    /// The match expression (simple form) or boolean condition
    /// (generic form).
    pub match_expr: LpgExpr,
    /// The result returned when this branch fires.
    pub result: LpgExpr,
    /// Source span.
    pub span: Span,
}

/// Binary operators in LPG expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LpgBinaryOp {
    /// `=`
    Eq,
    /// `<>`
    Ne,
    /// `<`
    Lt,
    /// `>`
    Gt,
    /// `<=`
    Le,
    /// `>=`
    Ge,
    /// `AND`
    And,
    /// `OR`
    Or,
    /// `XOR`
    Xor,
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `*`
    Mul,
    /// `/`
    Div,
    /// `%`
    Mod,
    /// `^`
    Pow,
    /// `STARTS WITH`
    StartsWith,
    /// `ENDS WITH`
    EndsWith,
    /// `CONTAINS`
    Contains,
}

/// Unary operators in LPG expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LpgUnaryOp {
    /// `NOT`
    Not,
    /// Unary `-`
    Minus,
    /// Unary `+`
    Plus,
}
