fn main() {
    use regex_dfa::parse_tree;
    use regex_syntax::ast::parse::Parser;

    let ast = Parser::new().parse("(a|b)*abb").unwrap();
    println!("ast: {:#?}", ast);

    let nfa = parse_tree::from_ast(&ast);
    println!("nfa: {:#?}", nfa);

    let dfa = nfa.as_dfa();
    println!("dfa: {:#?}", dfa);

    let minimized_dfa = dfa.as_minimized_dfa();
    println!("minimized_dfa: {:#?}", minimized_dfa);
}
