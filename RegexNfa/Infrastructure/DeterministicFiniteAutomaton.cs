
namespace RegexNfa.Infrastructure
{
    public class DeterministicFiniteAutomaton : Automaton
    {
        /// <summary>
        /// Traverses the DFA based on the input string
        /// </summary>
        /// <param name="input">The string to parse</param>
        /// <returns>True if the entire input string is accepted by the DFA</returns>
        public bool Parse(string input)
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
    }
}
