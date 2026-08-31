//! Structural tests: construction, routing, defaults, and equality.

use crate::{
    BranchScope,
    CreateClause,
    LabelMatch,
    LpgStatement,
    LpgWriteClause,
    MatchClause,
    NodePattern,
    PathMode,
    PatternElement,
    ReturnClause,
    ReturnItem,
};

fn node(var: &str, label: &str) -> NodePattern {
    NodePattern {
        variable: Some(var.to_owned()),
        labels: vec![label.to_owned()],
        label_match: LabelMatch::default(),
        properties: Vec::new(),
        has_property_block: false,
        span: 0..0,
    }
}

fn element(head: NodePattern) -> PatternElement {
    PatternElement {
        path_name: None,
        path_mode: PathMode::default(),
        head,
        chain: Vec::new(),
        span: 0..0,
    }
}

fn empty_return() -> ReturnClause {
    ReturnClause {
        distinct: false,
        star: true,
        items: Vec::<ReturnItem>::new(),
        group_by: Vec::new(),
        having: None,
        order_by: Vec::new(),
        skip: None,
        limit: None,
        span: 0..0,
    }
}

fn read_statement() -> LpgStatement {
    LpgStatement {
        call_clauses: Vec::new(),
        with_parts: Vec::new(),
        match_clause: Some(MatchClause {
            patterns: vec![element(node("n", "Person"))],
            where_clause: None,
            optional: false,
            span: 0..0,
        }),
        filter_clauses: Vec::new(),
        let_bindings: Vec::new(),
        for_clause: None,
        call_subqueries: Vec::new(),
        return_clause: empty_return(),
        branch_scope: None,
        write_clauses: Vec::new(),
        union_tail: Vec::new(),
        seed_relationships: Vec::new(),
        span: 0..0,
    }
}

#[test]
fn read_statement_is_not_a_write() {
    assert!(!read_statement().is_write());
}

#[test]
fn create_clause_makes_a_statement_a_write() {
    let mut statement = read_statement();
    statement.match_clause = None;
    statement.write_clauses = vec![LpgWriteClause::Create(CreateClause {
        patterns: vec![element(node("n", "Person"))],
        span: 0..0,
    })];
    assert!(statement.is_write());
}

#[test]
fn defaults_are_the_standard_semantics() {
    assert_eq!(PathMode::default(), PathMode::Standard);
    assert_eq!(LabelMatch::default(), LabelMatch::AllOf);
}

#[test]
fn statements_are_cloneable_and_comparable() {
    let statement = read_statement();
    let clone = statement.clone();
    assert_eq!(statement, clone);
}

#[test]
fn branch_scope_carries_the_literal_name() {
    let scope = BranchScope {
        name: "feature-x".to_owned(),
        span: 0..24,
    };
    let mut statement = read_statement();
    statement.branch_scope = Some(scope);
    assert_eq!(
        statement
            .branch_scope
            .as_ref()
            .map(|scope| scope.name.as_str()),
        Some("feature-x")
    );
}
