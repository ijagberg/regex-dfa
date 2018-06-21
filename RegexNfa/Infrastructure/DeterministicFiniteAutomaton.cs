
using System.Collections.Generic;
using System.Text;

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
                    foreach(Transition t in currentState.FromTransitions.Values)
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

    }
}
