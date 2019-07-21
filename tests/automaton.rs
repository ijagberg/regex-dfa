use regex_dfa::automaton;
use regex_dfa::automaton::{Automaton, StateMachine};

#[test]
fn test_concatenation_1() {
    let automaton = StateMachine::from_string("abc")
        .unwrap()
        .as_dfa()
        .as_minimized_dfa();
    assert!(automaton.match_whole("abc"));
    assert!(!automaton.match_whole("abcc"));
    assert!(!automaton.match_whole("ab"));
}

#[test]
fn test_concatenation_2() {
    let automaton = StateMachine::from_string("aaabc")
        .unwrap()
        .as_dfa()
        .as_minimized_dfa();
    assert!(automaton.match_whole("aaabc"));
    assert!(!automaton.match_whole("abcc"));
    assert!(!automaton.match_whole("ab"));
}

#[test]
fn test_alternation_1() {
    let automaton = StateMachine::from_string("a|b")
        .unwrap()
        .as_dfa()
        .as_minimized_dfa();
    assert!(automaton.match_whole("a"));
    assert!(automaton.match_whole("b"));
    assert!(!automaton.match_whole("ab"));
}

#[test]
fn test_grouping_1() {
    let automaton = StateMachine::from_string("a(bcd|efg)")
        .unwrap()
        .as_dfa()
        .as_minimized_dfa();
    assert!(automaton.match_whole("abcd"));
    assert!(automaton.match_whole("aefg"));
    assert!(!automaton.match_whole("abcdefg"));
    assert!(!automaton.match_whole("a"));
    assert!(!automaton.match_whole("abcde"));
}
