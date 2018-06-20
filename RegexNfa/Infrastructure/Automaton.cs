
using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text;

namespace RegexNfa.Infrastructure
{
    public class Automaton
    {
        public class State
        {
            public string Id { get; }
            public bool Accepting { get; set; }
            public Dictionary<string, Transition> FromTransitions;
            public Dictionary<string, Transition> ToTransitions;

            public State()
            {
                Id = Guid.NewGuid().ToString();
                Accepting = false;
                FromTransitions = new Dictionary<string, Transition>();
                ToTransitions = new Dictionary<string, Transition>();
            }

            public State(bool accepting) :
                this()
            {
                Accepting = accepting;
            }

            public void AddFromTransition(Transition transition)
            {
                try
                {
                    FromTransitions.Add(transition.Id, transition);
                } catch (ArgumentException ex)
                {
                    Debug.WriteLine(ex);
                    return;
                }

            }

            public void AddToTransition(Transition transition)
            {
                try
                {
                    ToTransitions.Add(transition.Id, transition);

                } catch (ArgumentException ex)
                {
                    Debug.WriteLine(ex);
                    return;
                }
            }

            public JObject ToJson()
            {
                JObject json = new JObject(
                    new JProperty("Id", Id),
                    new JProperty("Accepting", Accepting));
                return json;
            }

            public override string ToString()
            {
                return this.ToJson().ToString();
            }

        }

        public class Transition
        {
            public string Id { get; }
            public State FromState { get; set; }
            public State ToState { get; set; }
            public char Atom { get; set; }

            public Transition()
            {
                Id = Guid.NewGuid().ToString();
                FromState = new State();
                ToState = new State();
            }

            public Transition(State fromState, State toState, char atom = (char)0) :
                this()
            {
                FromState = fromState;
                ToState = toState;
                Atom = atom;
            }

            public JObject ToJson()
            {
                JObject json = new JObject(
                    new JProperty("Id", Id),
                    new JProperty("Atom", Atom.ToString()),
                    new JProperty("FromState", FromState.Id),
                    new JProperty("ToState", ToState.Id));
                return json;
            }

            public override string ToString()
            {
                return this.ToJson().ToString();
            }
        }

        public string Id { get; }
        public State StartState { get; set; }
        public SortedSet<char> Alphabet { get; set; }
        public Dictionary<string, State> States;
        public Dictionary<string, Transition> Transitions;
        public Dictionary<string, State> AcceptingStates;

        public Automaton()
        {
            Id = Guid.NewGuid().ToString();
            Alphabet = new SortedSet<char>();
            StartState = null;
            States = new Dictionary<string, State>();
            Transitions = new Dictionary<string, Transition>();
            AcceptingStates = new Dictionary<string, State>();
        }

        public void AddToAlphabet(params char[] atoms)
        {
            foreach(char c in atoms)
            {
                Alphabet.Add(c);
            }
        }

        public void AddState(State state)
        {
            if (!States.ContainsKey(state.Id))
            {
                States.Add(state.Id, state);
                if (state.Accepting)
                {
                    AcceptingStates.Add(state.Id, state);
                }
                foreach (Transition transition in state.FromTransitions.Values)
                {
                    AddTransition(transition);
                }
                foreach (Transition transition in state.ToTransitions.Values)
                {
                    AddTransition(transition);
                }
            }
        }

        public void AddTransition(Transition transition)
        {
            if (!Transitions.ContainsKey(transition.Id))
            {
                if (!States.ContainsKey(transition.FromState.Id) || !States.ContainsKey(transition.ToState.Id))
                {
                    // Follow the rules
                    return;
                }
                transition.FromState.AddFromTransition(transition);
                transition.ToState.AddToTransition(transition);
                Transitions.Add(transition.Id, transition);
            }
        }

        public void AddTransition(State from, State to, char data = '\0')
        {
            Transition transition = new Transition(from, to, data);
            AddTransition(transition);
        }

        public override string ToString()
        {
            JObject json = new JObject(new JProperty("StartState", StartState.ToJson()),
                new JProperty("States", new JArray(
                    from s in States.Values
                    orderby s.Id
                    select s.ToJson()
                )),
                new JProperty("Transitions", new JArray(
                    from t in Transitions.Values
                    orderby t.FromState.Id
                    select t.ToJson()
                )));
            return json.ToString();
        }

    }

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
            //

            Dictionary<string, HashSet<string>> stateSets = new Dictionary<string, HashSet<string>>(); 
            stateSets.Add(dfaStartState.Id, dfaStartStateSet);

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
                    foreach(KeyValuePair<string, HashSet<string>> entry in stateSets)
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

    public class DeterministicFiniteAutomaton : Automaton
    {

    }
}
