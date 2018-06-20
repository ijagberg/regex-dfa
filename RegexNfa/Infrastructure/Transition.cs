using Newtonsoft.Json.Linq;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace RegexNfa.Infrastructure
{
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
}
