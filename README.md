# regex-dfa
Library for parsing regular expressions into dfas (deterministic finite automatons)

# Explanation
For every regular expression there exists a dfa that matches on the same strings the expression would match. This library allows you to generate this dfa and export it to an image.

There are two parts to the library. The first part is the ParseTree enum, which allows you to parse regular expressions into tree structures that are easy to display or perform operations upon. The second part is the Automaton struct, which exposes methods that allow you to traverse a dfa generated from a ParseTree. 

# Example
```rust
let parsed_input = ParseTree::from("Hello(, World)?!")
let dfa = Automaton::from(&parsed_input);
assert!(dfa.match_whole("Hello, World!");
assert!(dfa.match_whole("Hello!");
```
