
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
            foreach (char c in atoms)
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
            JObject json = new JObject(new JProperty("Alphabet", new JArray(
                                            from c in Alphabet
                                            orderby c
                                            select c.ToString())),
                new JProperty("StartState", StartState.ToJson()),
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


}
