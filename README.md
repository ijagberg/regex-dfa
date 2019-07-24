# regex-dfa
Library for parsing regular expressions into dfas (deterministic finite automatons)

# Explanation
For every regular expression there exists a dfa that matches on the same strings the expression would match. This library allows you to generate this dfa.

# Usage
```cargo run --example from_args "((a|b)*c)+``` will generate output that can be rendered by various GraphViz tools:

![Automaton that simulates the regular expression "((a|b)\*c)+"](https://i.imgur.com/gqHzBGO.png)
