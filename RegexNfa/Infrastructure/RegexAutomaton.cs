using System.Collections.Generic;

namespace RegexNfa.Infrastructure
{
    public class RegexAutomaton : Automaton
    {
        public State EndState { get; set; }

        public RegexAutomaton() :
            base()
        {
            EndState = null;
        }

        public void AddStatesAndTransitions(params RegexAutomaton[] automatons)
        {
            foreach (RegexAutomaton automaton in automatons)
            {
                // Get all the states
                foreach (State state in automaton.States.Values)
                {
                    if (state.Accepting)
                    {
                        state.Accepting = false;
                    }
                    AddState(state);
                }

                // Get all the transitions
                foreach (Transition transition in automaton.Transitions.Values)
                {
                    // I don't think this should ever run but might as well to be sure
                    AddTransition(transition);
                }
            }
        }

        public static DeterministicFiniteAutomaton ConvertToDfa(RegexAutomaton regex)
        {
            HashSet<string> dfaStartStateSet = EpsilonClosure(regex, regex.StartState);

            // Set start state in the DFA
            DeterministicFiniteAutomaton dfa = new DeterministicFiniteAutomaton();
            State dfaStartState = new State();
            dfa.AddState(dfaStartState);
            dfa.StartState = dfaStartState;
            dfa.Alphabet = regex.Alphabet;
            //

            Dictionary<string, HashSet<string>> stateSets = new Dictionary<string, HashSet<string>>
            {
                { dfaStartState.Id, dfaStartStateSet }
            };

            Stack<string> unvisited = new Stack<string>();

            unvisited.Push(dfaStartState.Id); // States in the resulting DFA that we haven't fully built yet

            while (unvisited.Count > 0)
            {
                string fromStateId = unvisited.Pop();
                HashSet<string> fromStateSet = stateSets[fromStateId];

                foreach (char atom in regex.Alphabet)
                {
                    HashSet<string> fromReachableSet = EpsilonClosure(regex, Reachable(regex, fromStateSet, atom));

                    if (fromReachableSet.Count == 0)
                    {
                        // This is the empty set, we don't really need to add it to the structure.
                        continue;
                    }

                    // Compare currentReachable with the current states in the dfa
                    string toStateId = null;
                    foreach (KeyValuePair<string, HashSet<string>> entry in stateSets)
                    {
                        if (fromReachableSet.SetEquals(entry.Value))
                        {
                            toStateId = entry.Key;
                            break;
                        }
                    }

                    if (toStateId == null)
                    {
                        // Create a new state in the dfa
                        State newState = new State();

                        // Is the new state accepting?
                        foreach (string state in fromReachableSet)
                        {
                            if (regex.States[state].Accepting)
                            {
                                newState.Accepting = true;
                                break;
                            }
                        }

                        dfa.AddState(newState);
                        stateSets.Add(newState.Id, fromReachableSet);

                        toStateId = newState.Id;

                        unvisited.Push(toStateId);
                    }

                    // Regardless, add a transition between fromState and toState
                    dfa.AddTransition(dfa.States[fromStateId], dfa.States[toStateId], atom);

                }
            }

            return dfa;
        }

        private static HashSet<string> EpsilonClosure(RegexAutomaton regex, State fromState)
        {
            HashSet<string> fromStates = new HashSet<string>
            {
                fromState.Id
            };
            return EpsilonClosure(regex, fromStates);
        }

        private static HashSet<string> EpsilonClosure(RegexAutomaton regex, HashSet<string> fromStates)
        {
            char epsilon = (char)0;
            HashSet<string> closure = new HashSet<string>();
            Stack<string> unvisited = new Stack<string>();
            foreach (string fromStateId in fromStates)
            {
                if (!closure.Contains(fromStateId))
                {
                    unvisited.Push(fromStateId);
                }
                while (unvisited.Count > 0)
                {
                    State current = regex.States[unvisited.Pop()];
                    closure.Add(current.Id);
                    foreach (Transition t in current.FromTransitions.Values)
                    {
                        if (t.Atom == epsilon && !closure.Contains(t.ToState.Id))
                        {
                            unvisited.Push(t.ToState.Id);
                        }
                    }
                }
            }
            return closure;
        }

        private static HashSet<string> Reachable(RegexAutomaton regex, HashSet<string> fromStates, char atom)
        {
            HashSet<string> reachable = new HashSet<string>();
            foreach (string fromStateId in fromStates)
            {
                State fromState = regex.States[fromStateId];
                foreach (Transition transition in fromState.FromTransitions.Values)
                {
                    if (transition.Atom == atom && !reachable.Contains(transition.ToState.Id))
                    {
                        reachable.Add(transition.ToState.Id);
                    }
                }
            }
            // Returning an empty HashSet means we go to the empty state on this atom, ending the parser
            return reachable;
        }
    }
}