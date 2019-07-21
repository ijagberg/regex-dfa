use regex_dfa::automaton::Automaton;

#[test]
fn test_concatenation_1() {
    let automaton = Automaton::from_string("abc").unwrap().into_min_dfa();
    assert!(automaton.match_whole("abc"));
    assert!(!automaton.match_whole("abcc"));
    assert!(!automaton.match_whole("ab"));
}

#[test]
fn test_concatenation_2() {
    let automaton = Automaton::from_string("aaabc").unwrap().into_min_dfa();
    assert!(automaton.match_whole("aaabc"));
    assert!(!automaton.match_whole("abcc"));
    assert!(!automaton.match_whole("ab"));
}

#[test]
fn test_alternation_1() {
    let automaton = Automaton::from_string("a|b").unwrap().into_min_dfa();
    assert!(automaton.match_whole("a"));
    assert!(automaton.match_whole("b"));
    assert!(!automaton.match_whole("ab"));
}

#[test]
fn test_grouping_1() {
    let automaton = Automaton::from_string("a(bcd|efg)").unwrap().into_min_dfa();
    assert!(automaton.match_whole("abcd"));
    assert!(automaton.match_whole("aefg"));
    assert!(!automaton.match_whole("abcdefg"));
    assert!(!automaton.match_whole("a"));
    assert!(!automaton.match_whole("abcde"));
}

#[test]
fn test_star_1() {
    let automaton = Automaton::from_string("a*").unwrap().into_min_dfa();
    assert!(automaton.match_whole(""));
    assert!(automaton.match_whole("a"));
    assert!(automaton.match_whole("aa"));
    assert!(automaton.match_whole("aaa"));
    assert!(!automaton.match_whole("aaab"));
}

#[test]
fn test_plus_1() {
    let automaton = Automaton::from_string("a+").unwrap().into_min_dfa();
    assert!(!automaton.match_whole(""));
    assert!(automaton.match_whole("a"));
    assert!(automaton.match_whole("aa"));
    assert!(automaton.match_whole("aaa"));
    assert!(!automaton.match_whole("aaab"));
}

#[test]
fn test_question_1() {
    let automaton = Automaton::from_string("a?").unwrap().into_min_dfa();
    assert!(automaton.match_whole(""));
    assert!(automaton.match_whole("a"));
    assert!(!automaton.match_whole("aa"));
}
