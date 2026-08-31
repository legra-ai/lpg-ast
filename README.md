# lpg-ast

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![CI][ci-badge]][ci-url]
[![License][license-badge]][license-url]
[![Downloads][downloads-badge]][downloads-url]

Language-neutral abstract syntax tree for Labeled Property Graph queries.

`lpg-ast` is the shared intermediate representation that both a Cypher
parser and a GQL parser can lower into, so everything downstream of parsing
— algebra translation, planning, execution — handles one AST regardless of
which language the query was written in:

```text
Cypher source ──> Cypher parser ──┐
                                  ├──> LpgStatement ──> algebra translator ──> logical plan
GQL source    ──> GQL parser    ──┘
```

The crate contains only types: no parser, no translator, no execution. It
has zero dependencies.

## Shape

An [`LpgStatement`] is either a read (`MATCH` / `WITH` / `RETURN`, with
`CALL` procedures and subqueries, `UNION` tails, GQL `FILTER` / `LET` /
`FOR`) or a write (`CREATE` / `DELETE` / `SET` / `MERGE` / `REMOVE` /
`FOREACH`), routed by `LpgStatement::is_write()`. Every node carries a
`Span` (byte range into the source text) for diagnostics.

Pattern types cover the shared Cypher/GQL surface: node and relationship
patterns with label conjunction (`:A:B`) and disjunction (`:A|B`),
direction, quantifiers (`{min,max}` / `*min..max`), named paths,
`shortestPath` / `allShortestPaths` modes, and inline property blocks.
Expressions cover literals, operators, function calls, list and pattern
comprehensions, list predicates (`ANY` / `ALL` / `NONE` / `SINGLE`),
`reduce`, `CASE`, `EXISTS { … }`, and `COUNT { … }` subqueries.

```rust
use lpg_ast::{LabelMatch, MatchClause, NodePattern, PathMode, PatternElement};

let person = NodePattern {
    variable: Some("n".to_owned()),
    labels: vec!["Person".to_owned()],
    label_match: LabelMatch::default(),
    properties: Vec::new(),
    has_property_block: false,
    span: 6..17,
};
let clause = MatchClause {
    patterns: vec![PatternElement {
        path_name: None,
        path_mode: PathMode::default(),
        head: person,
        chain: Vec::new(),
        span: 6..17,
    }],
    where_clause: None,
    optional: false,
    span: 0..17,
};
assert_eq!(clause.patterns.len(), 1);
```

## Extensions

Alongside the standard Cypher and GQL surface, the AST deliberately carries
extensions for branch-versioned graph stores — most visibly
[`BranchScope`]: `USE BRANCH "name" { … }` evaluates the wrapped statement
against a named branch, the way git checks out a branch. Stores without
branching semantics can reject the node at translation time.

[`LpgStatement`]: https://docs.rs/lpg-ast/latest/lpg_ast/struct.LpgStatement.html
[`BranchScope`]: https://docs.rs/lpg-ast/latest/lpg_ast/struct.BranchScope.html

## License

Licensed under either of:

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE));
- MIT License ([`LICENSE-MIT`](LICENSE-MIT)).

## Links

[crates-badge]: https://img.shields.io/crates/v/lpg-ast.svg
[crates-url]: https://crates.io/crates/lpg-ast
[docs-badge]: https://docs.rs/lpg-ast/badge.svg
[docs-url]: https://docs.rs/lpg-ast
[ci-badge]: https://github.com/legra-ai/lpg-ast/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/legra-ai/lpg-ast/actions/workflows/ci.yml
[license-badge]: https://img.shields.io/crates/l/lpg-ast.svg
[license-url]: https://github.com/legra-ai/lpg-ast/blob/main/LICENSE-APACHE
[downloads-badge]: https://img.shields.io/crates/d/lpg-ast.svg
[downloads-url]: https://crates.io/crates/lpg-ast
