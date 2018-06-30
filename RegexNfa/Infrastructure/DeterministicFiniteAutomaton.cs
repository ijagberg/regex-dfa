using System;
using System.Collections.Generic;
using System.Linq;

namespace RegexNfa.Infrastructure
{
    public class DeterministicFiniteAutomaton : Automaton
    {
        /// <summary>
        /// Parses an entire input string through the automaton
        /// </summary>
        /// <param name="input">The string to parse</param>
        /// <returns>True if the entire input string is accepted by the DFA</returns>
        public bool MatchEntire(string input)
        {
            State currentState = StartState;
            foreach (char c in input)
            {
                // Check if there is any transition from the current state with an atom that matches c
                bool transitionExists = false;
                foreach (Transition t in currentState.FromTransitions.Values)
                {
                    if (t.Atom == c)
                    {
                        // Traverse to the toState
                        currentState = t.ToState;
                        transitionExists = true;
                        break;
                    }
                }

                if (!transitionExists)
                {
                    // Parsing this character is impossible
                    return false;
                }
            }

            // If the state we are in after parsing the entire input is accepting, then the input is accepted
            return currentState.Accepting;
        }

        /// <summary>
        /// Parses an input string through the automaton
        /// </summary>
        /// <param name="input"></param>
        /// <returns>The index of the first match</returns>
        public IList<string> MatchSubstrings(string input)
        {
            IList<string> matchingSubstrings = new List<string>();

            // Special case for the matching of the empty string
            if (StartState.Accepting)
            {
                matchingSubstrings.Add(string.Empty);
            }

            // Test each starting point
            State currentState;
            for (int i = 0; i < input.Length; i++)
            {
                currentState = StartState;
                for (int j = i; j < input.Length; j++)
                {
                    bool transitionExists = false;
                    foreach (Transition t in currentState.FromTransitions.Values)
                    {
                        if (t.Atom == input[j])
                        {
                            currentState = t.ToState;
                            transitionExists = true;
                            break;
                        }
                    }

                    if (!transitionExists)
                    {
                        break;
                    }

                    if (currentState.Accepting)
                    {
                        matchingSubstrings.Add(input.Substring(i, j - i + 1));
                    }
                }
            }

            return matchingSubstrings;
        }

        /// <summary>
        /// Returns a new DFA which accepts any string that <i>isn't</i> accepted by the given DFA
        /// </summary>
        /// <param name="dfa"></param>
        /// <returns></returns>
        public static DeterministicFiniteAutomaton Negate(DeterministicFiniteAutomaton dfa)
        {
            DeterministicFiniteAutomaton negatedDfa = dfa;

            foreach(State state in negatedDfa.States.Values)
            {
                state.Accepting = !state.Accepting;
            }

            return negatedDfa;
        }

        /// <summary>
        /// Multiplies to DFA:s to create a product DFA that only matches input that <i>both</i> factor DFA:s would match
        /// </summary>
        /// <param name="a"></param>
        /// <param name="b"></param>
        /// <returns></returns>
        public static DeterministicFiniteAutomaton operator *(DeterministicFiniteAutomaton a, DeterministicFiniteAutomaton b)
        {
            // Start by intersecting the two alphabets 
            SortedSet<char> alphabetA = a.Alphabet;
            SortedSet<char> alphabetB = b.Alphabet;
            SortedSet<char> newAlphabet = new SortedSet<char>((from atom1 in alphabetA
                               select atom1).Intersect(from atom2 in alphabetB select atom2));

            DeterministicFiniteAutomaton product = new DeterministicFiniteAutomaton
            {
                Alphabet = newAlphabet
            };
            Tuple<string, string> productStartStateSet = new Tuple<string, string>(a.StartState.Id, b.StartState.Id);
            State productStartState = new State();
            product.AddState(productStartState);
            product.StartState = productStartState;

            Dictionary<string, Tuple<string, string>> stateSets = new Dictionary<string, Tuple<string, string>>
            {
                { productStartState.Id, productStartStateSet }
            };

            Stack<string> unvisited = new Stack<string>();
            unvisited.Push(productStartState.Id);

            while (unvisited.Count > 0)
            {
                string fromStateId = unvisited.Pop();
                Tuple<string, string> fromStateSet = stateSets[fromStateId];

                foreach (char atom in product.Alphabet)
                {
                    State toState1 = null;
                    State toState2 = null;
                    foreach (Transition t1 in a.States[fromStateSet.Item1].FromTransitions.Values)
                    {
                        if (t1.Atom == atom)
                        {
                            toState1 = t1.ToState;
                            break;
                        }
                    }
                    foreach (Transition t2 in b.States[fromStateSet.Item2].FromTransitions.Values)
                    {
                        if (t2.Atom == atom)
                        {
                            toState2 = t2.ToState;
                            break;
                        }
                    }

                    if (toState1 == null || toState2 == null)
                    {
                        // One of the two transitions go to the empty state, meaning they will go to the empty state in the product as well
                        continue;
                    } else
                    {
                        Tuple<string, string> toStateSet = new Tuple<string, string>(toState1.Id, toState2.Id);
                        string toStateId = null;
                        foreach (KeyValuePair<string, Tuple<string, string>> entry in stateSets)
                        {
                            // Check if we have added this stateSet earlier
                            if (entry.Value == toStateSet)
                            {
                                toStateId = entry.Key;
                                break;
                            }
                        }

                        if (toStateId == null)
                        {
                            // Add a new state in the product DFA
                            State newState = new State
                            {
                                Accepting = (toState1.Accepting && toState2.Accepting)
                            };

                            toStateId = newState.Id;
                            product.AddState(newState);
                            stateSets.Add(toStateId, toStateSet);

                            unvisited.Push(toStateId);
                        }

                        // Add transition to the new state
                        product.AddTransition(product.States[fromStateId], product.States[toStateId], atom);
                    }
                }
            }

            return product;
        }

    }
}
