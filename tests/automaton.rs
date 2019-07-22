use regex_dfa::automaton::Automaton;

#[test]
fn test_concatenation_whole_1() {
    let automaton = Automaton::from_string("abc").unwrap().into_min_dfa();
    assert!(automaton.match_whole("abc"));
    assert!(!automaton.match_whole("abcc"));
    assert!(!automaton.match_whole("ab"));
}

#[test]
fn test_concatenation_whole_2() {
    let automaton = Automaton::from_string("aaabc").unwrap().into_min_dfa();
    assert!(automaton.match_whole("aaabc"));
    assert!(!automaton.match_whole("abcc"));
    assert!(!automaton.match_whole("ab"));
}

#[test]
fn test_concatenation_substrings_1() {
    let automaton = Automaton::from_string("a+").unwrap().into_min_dfa();
    println!("{}", automaton.to_dot_format());
    let input_str = "aaa";
    assert_eq!(
        automaton.match_substrings(input_str),
        vec![(0..1), (0..2), (0..3), (1..2), (1..3), (2..3)]
    );
}

#[test]
fn test_concatenation_substrings_2() {
    let automaton = Automaton::from_string("a*").unwrap().into_min_dfa();
    println!("{}", automaton.to_dot_format());
    let input_str = "aaa";
    assert_eq!(
        automaton.match_substrings(input_str),
        vec![
            (0..0),
            (0..1),
            (0..2),
            (0..3),
            (1..1),
            (1..2),
            (1..3),
            (2..2),
            (2..3)
        ]
    );
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

#[test]
fn test_literal_range_1() {
    let automaton = Automaton::from_string("[a-z]+").unwrap().into_min_dfa();
    for atom in (b'a'..=b'z').map(char::from) {
        assert!(automaton.match_whole(&atom.to_string()));
    }
    assert!(automaton.match_whole("abcdefghijk"));
    assert!(!automaton.match_whole("1"));
}
