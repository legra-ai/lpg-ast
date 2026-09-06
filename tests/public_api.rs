//! Public-API integration test: the AST types construct and compare from
//! the shipped crate.

use lpg_ast::{
    LabelMatch,
    MatchClause,
    NodePattern,
    PathMode,
    PatternElement,
};

#[test]
fn a_match_clause_holds_its_pattern_and_spans() {
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
            head: person.clone(),
            chain: Vec::new(),
            span: 6..17,
        }],
        where_clause: None,
        optional: false,
        span: 0..17,
    };
    assert_eq!(clause.patterns.len(), 1);
    assert_eq!(clause.patterns[0].head, person);
    assert_eq!(clause.patterns[0].head.variable.as_deref(), Some("n"));
    assert_eq!(clause.span, 0..17);
    assert!(!clause.optional);
}
