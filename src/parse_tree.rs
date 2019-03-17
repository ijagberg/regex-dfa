use super::automaton::Automaton;
use regex_syntax::ast::Ast;

pub fn from_ast(ast_tree: &Ast) -> Automaton {
    match ast_tree {
        Ast::Concat(ast) => build_concatenation(ast),
        Ast::Repetition(ast) => build_repetition(ast),
        Ast::Literal(ast) => build_literal(ast.c),
        Ast::Alternation(ast) => build_alternation(ast),
        Ast::Group(ast) => from_ast(&ast.ast),
        x => panic!("No support for {:?} (yet)", x),
    }
}

fn build_concatenation(concat_ast: &regex_syntax::ast::Concat) -> Automaton {
    let mut concat_automaton = Automaton::new();
    let concat_start_state = concat_automaton.add_state();
    concat_automaton.set_start_state(concat_start_state);

    let mut concat_end_state = concat_start_state;

    for append_ast in &concat_ast.asts {
        println!("concat_end_state: {}", concat_end_state);
        let append_automaton = from_ast(append_ast);
        assert_eq!(append_automaton.accepting_states.len(), 1);
        let append_start_state = append_automaton.start_state.unwrap();
        let append_end_state = *append_automaton.accepting_states.iter().next().unwrap();
        let concat_append_offset = concat_automaton.states;
        concat_automaton.add_states_and_transitions(append_automaton);

        // Add transition from previous append_automaton's end state to current append_automaton's start state
        concat_automaton.add_transition(
            concat_end_state,
            append_start_state + concat_append_offset,
            None,
        );

        // Change end state to be the current append_automaton's end state
        concat_end_state = append_end_state + concat_append_offset;
        concat_automaton.clear_accepting();
        concat_automaton.set_accepting(concat_end_state, true);
    }

    concat_automaton
}

fn build_repetition(repetition_ast: &regex_syntax::ast::Repetition) -> Automaton {
    use regex_syntax::ast::RepetitionKind;

    let mut repetition_automaton = Automaton::new();
    let repetition_start_state = repetition_automaton.add_state();
    let repetition_end_state = repetition_automaton.add_state();
    let repetition_to_inner_offset = repetition_automaton.states;

    let inner_automaton = from_ast(&repetition_ast.ast);
    assert_eq!(inner_automaton.accepting_states.len(), 1);
    let inner_automaton_start_state = inner_automaton.start_state.unwrap();
    let inner_automaton_end_state = *inner_automaton.accepting_states.iter().next().unwrap();
    repetition_automaton.add_states_and_transitions(inner_automaton);

    // Add transition from repetition_automaton's start state to inner_automaton's start state
    repetition_automaton.add_transition(
        repetition_start_state,
        inner_automaton_start_state + repetition_to_inner_offset,
        None,
    );

    // Add transition from inner_automaton's end state to repetition_automaton's end state
    repetition_automaton.add_transition(
        inner_automaton_end_state + repetition_to_inner_offset,
        repetition_end_state,
        None,
    );

    match repetition_ast.op.kind {
        RepetitionKind::OneOrMore => {
            // Add transition from repetition_automaton's end state to repetition_automaton's start state
            repetition_automaton.add_transition(repetition_end_state, repetition_start_state, None);
        }
        RepetitionKind::ZeroOrMore => {
            // Add transition from repetition_automaton's start state to repetition_automaton's end state (for Zero)
            repetition_automaton.add_transition(repetition_start_state, repetition_end_state, None);
            // Add transition from repetition_automaton's end state to repetition_automaton's start state
            repetition_automaton.add_transition(repetition_end_state, repetition_start_state, None);
        }
        RepetitionKind::ZeroOrOne => {
            // Add transition from repetition_automaton's start state to repetition_automaton's end state
            repetition_automaton.add_transition(repetition_start_state, repetition_end_state, None);
        }
        RepetitionKind::Range(_) => {
            panic!("RepetitionKind::Range is not supported yet!");
        }
    }

    repetition_automaton.set_start_state(repetition_start_state);
    repetition_automaton.clear_accepting();
    repetition_automaton.set_accepting(repetition_end_state, true);

    repetition_automaton
}

fn build_alternation(alternation_ast: &regex_syntax::ast::Alternation) -> Automaton {
    let mut alternation_automaton = Automaton::new();
    let alternation_automaton_start_state = alternation_automaton.add_state();
    let alternation_automaton_end_state = alternation_automaton.add_state();

    for alternative_ast in &alternation_ast.asts {
        let alternative_automaton = from_ast(alternative_ast);
        assert_eq!(alternative_automaton.accepting_states.len(), 1);

        let alternative_automaton_start_state = alternative_automaton.start_state.unwrap();
        let alternative_automaton_end_state = *alternative_automaton
            .accepting_states
            .iter()
            .next()
            .unwrap();
        let alternation_to_alternative_offset = alternation_automaton.states;
        alternation_automaton.add_states_and_transitions(alternative_automaton);

        // Add transition from alternation_automaton's start state to alternative_automaton's start state
        alternation_automaton.add_transition(
            alternation_automaton_start_state,
            alternative_automaton_start_state + alternation_to_alternative_offset,
            None,
        );

        // Add transition from alternative_automaton's end state to alternation_automaton's end state
        alternation_automaton.add_transition(
            alternative_automaton_end_state + alternation_to_alternative_offset,
            alternation_automaton_end_state,
            None,
        );
    }

    alternation_automaton.set_start_state(alternation_automaton_start_state);
    alternation_automaton.clear_accepting();
    alternation_automaton.set_accepting(alternation_automaton_end_state, true);

    alternation_automaton
}

fn build_literal(c: char) -> Automaton {
    let mut literal_automaton = Automaton::new();
    let start_state = literal_automaton.add_state();
    let end_state = literal_automaton.add_state();
    literal_automaton.set_accepting(end_state, true);
    literal_automaton.set_start_state(start_state);
    literal_automaton.add_transition(start_state, end_state, Some(c));
    literal_automaton
}

#[test]
fn alternation() {
    use regex_syntax::ast::parse::Parser;
    let alternation_ast = Parser::new().parse("a|b|c").unwrap();
    // println!("alternation_ast: {:#?}", alternation_ast);

    let automaton = from_ast(&alternation_ast).as_dfa().as_minimized_dfa();
    assert!(automaton.match_whole("a"));
    assert!(automaton.match_whole("b"));
    assert!(automaton.match_whole("c"));
    assert!(!automaton.match_whole("abc"));
    assert!(!automaton.match_whole("d"));
}

#[test]
fn concat() {
    use regex_syntax::ast::parse::Parser;
    let concat_ast = Parser::new().parse("abc").unwrap();
    // println!("concat_ast: {:#?}", concat_ast);

    let automaton = from_ast(&concat_ast).as_dfa().as_minimized_dfa();
    assert!(automaton.match_whole("abc"));
    assert!(!automaton.match_whole("a"));
    assert!(!automaton.match_whole("b"));
    assert!(!automaton.match_whole("c"));
    assert!(!automaton.match_whole("abcd"));
}

#[test]
fn repetition_zero_or_more() {
    use regex_syntax::ast::parse::Parser;
    let repetition_ast = Parser::new().parse("a*").unwrap();
    // println!("repetition_ast: {:#?}", repetition_ast);

    let repetition_nfa = from_ast(&repetition_ast);
    // println!("repetition_nfa: {:#?}", repetition_nfa);

    let repetition_dfa = repetition_nfa.as_dfa();
    // println!("repetition_dfa: {:#?}", repetition_dfa);

    let repetition_minimized_dfa = repetition_dfa.as_minimized_dfa();
    // println!("repetition_minimized_dfa: {:#?}", repetition_minimized_dfa);
    assert!(repetition_minimized_dfa.match_whole("a"));
    assert!(repetition_minimized_dfa.match_whole("aa"));
    assert!(repetition_minimized_dfa.match_whole("aaa"));
    assert!(repetition_minimized_dfa.match_whole(""));
    assert!(!repetition_minimized_dfa.match_whole("b"));
}

#[test]
fn repetition_zero_or_one() {
    use regex_syntax::ast::parse::Parser;
    let repetition_ast = Parser::new().parse("a?").unwrap();
    // println!("repetition_ast: {:#?}", repetition_ast);

    let repetition_nfa = from_ast(&repetition_ast);
    // println!("repetition_nfa: {:#?}", repetition_nfa);

    let repetition_dfa = repetition_nfa.as_dfa();
    // println!("repetition_dfa: {:#?}", repetition_dfa);

    let repetition_minimized_dfa = repetition_dfa.as_minimized_dfa();
    // println!("repetition_minimized_dfa: {:#?}", repetition_minimized_dfa);
    assert!(repetition_minimized_dfa.match_whole(""));
    assert!(repetition_minimized_dfa.match_whole("a"));
    assert!(!repetition_minimized_dfa.match_whole("aa"));
    assert!(!repetition_minimized_dfa.match_whole("b"));
}

#[test]
fn repetition_one_or_more() {
    use regex_syntax::ast::parse::Parser;
    let repetition_ast = Parser::new().parse("a+").unwrap();
    // println!("repetition_ast: {:#?}", repetition_ast);

    let repetition_nfa = from_ast(&repetition_ast);
    // println!("repetition_nfa: {:#?}", repetition_nfa);

    let repetition_dfa = repetition_nfa.as_dfa();
    // println!("repetition_dfa: {:#?}", repetition_dfa);

    let repetition_minimized_dfa = repetition_dfa.as_minimized_dfa();
    // println!("repetition_minimized_dfa: {:#?}", repetition_minimized_dfa);
    assert!(repetition_minimized_dfa.match_whole("a"));
    assert!(repetition_minimized_dfa.match_whole("aa"));
    assert!(repetition_minimized_dfa.match_whole("aaa"));
    assert!(!repetition_minimized_dfa.match_whole(""));
    assert!(!repetition_minimized_dfa.match_whole("b"));
}

#[test]
fn group() {
    use regex_syntax::ast::parse::Parser;
    let group_ast = Parser::new().parse("(ab)*").unwrap();
    println!("group_ast: {:#?}", group_ast);

    let group_nfa = from_ast(&group_ast);
    println!("group_nfa: {:#?}", group_nfa);

    let group_dfa = group_nfa.as_dfa();
    println!("group_dfa: {:#?}", group_dfa);

    let group_minimized_dfa = group_dfa.as_minimized_dfa();
    println!("group_minimized_dfa: {:#?}", group_minimized_dfa);
    assert!(group_minimized_dfa.match_whole("ab"));
    assert!(group_minimized_dfa.match_whole("abab"));
    assert!(group_minimized_dfa.match_whole(""));
    assert!(!group_minimized_dfa.match_whole("b"));
    assert!(!group_minimized_dfa.match_whole("aba"));
}
