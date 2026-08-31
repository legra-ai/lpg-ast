#![doc = include_str!("../README.md")]

//! Language-neutral LPG Abstract Syntax Tree types.
//!
//! These types represent the structure of a parsed Labeled Property
//! Graph query after lowering from a language-specific CST. Both the
//! Cypher and GQL parsers produce this same AST, which the algebra
//! translator then converts into a logical plan.

mod binding;
mod branch;
mod call;
mod expr;
mod match_pattern;
mod return_clause;
mod span;
mod statement;
mod with;
mod write;

#[cfg(test)]
mod tests;

pub use binding::{
    ForClause,
    LetBinding,
    PropertyEntry,
};
pub use branch::BranchScope;
pub use call::{
    CallClause,
    CallSubquery,
    YieldItem,
};
pub use expr::{
    CaseBranch,
    LpgBinaryOp,
    LpgExpr,
    LpgListPredicateKind,
    LpgUnaryOp,
    MapEntry,
};
pub use match_pattern::{
    LabelMatch,
    MatchClause,
    NodePattern,
    PathMode,
    PatternElement,
    Quantifier,
    RelDirection,
    RelationshipPattern,
    SeedRelationship,
};
pub use return_clause::{
    OrderItem,
    ReturnClause,
    ReturnItem,
};
pub use span::Span;
pub use statement::{
    LpgStatement,
    UnionBranch,
};
pub use with::{
    WithClause,
    WithPart,
};
pub use write::{
    CreateClause,
    DeleteClause,
    ForeachClause,
    LpgWriteClause,
    MergeClause,
    RemoveClause,
    RemoveItem,
    SetClause,
    SetItem,
};
