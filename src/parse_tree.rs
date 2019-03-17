use super::automaton::Automaton;
use regex_syntax::ast::parse::Parser;
use regex_syntax::ast::Ast;

pub fn from_ast(ast_tree: &Ast) -> Automaton {
    match ast_tree {
        Ast::Concat(concat) => build_concatenation(concat),
        Ast::Repetition(repetition) => build_repetition(repetition),
        Ast::Literal(literal) => build_literal(literal.c),
        Ast::Alternation(alternation) => build_alternation(alternation),
        _ => panic!("No support for this type of AST yet"),
    }
}

fn build_concatenation(concat_ast: &regex_syntax::ast::Concat) -> Automaton {
    let mut concat_automaton = Automaton::new();
    let concat_automaton_start_state = concat_automaton.add_state();
    let concat_automaton_end_state = concat_automaton.add_state();

    let mut asts_iter = concat_ast.asts.iter();

    // Add first append_automaton to the concatenation
    let first_append_automaton = from_ast(asts_iter.next().unwrap());
    


    concat_automaton.set_start_state(concat_automaton_start_state);
    concat_automaton.clear_accepting();
    concat_automaton.set_accepting(concat_automaton_end_state, true);

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
fn print_alternation() {
    let alternation_ast = Parser::new().parse("a|b|c").unwrap();
    println!("{:#?}", alternation_ast);

    let automaton = from_ast(&alternation_ast);
    println!("{:#?}", automaton);

    let dfa = automaton.as_dfa();
    println!("{:#?}", dfa);

    let minimized_dfa = dfa.as_minimized_dfa();
    println!("{:#?}", minimized_dfa);
}

#[test]
fn print_concat() {
    let concat_ast = Parser::new().parse("abc").unwrap();
    println!("{:#?}", concat_ast);

    let automaton = from_ast(&concat_ast);
    println!("{:#?}", automaton);

    let dfa = automaton.as_dfa();
    println!("{:#?}", dfa);

    let minimized_dfa = dfa.as_minimized_dfa();
    println!("{:#?}", minimized_dfa);
}
