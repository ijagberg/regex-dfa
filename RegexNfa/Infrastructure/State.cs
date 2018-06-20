using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Diagnostics;

namespace RegexNfa.Infrastructure
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
}