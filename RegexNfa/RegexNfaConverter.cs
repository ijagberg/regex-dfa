using RegexNfa.Infrastructure;
using RegexParser.Infrastructure;

namespace RegexNfa
{
    public class RegexNfaConverter
    {

        public RegexAutomaton ConvertToNfa(ParseTree tree)
        {
            RegexAutomaton automaton = BuildFromTree(tree);
            return automaton;
        }

        private RegexAutomaton BuildFromTree(ParseTree tree)
        {
            switch (tree.Type)
            {
                case NodeType.Concatenation:
                    return BuildFromConcatenation((ConcatenationTree)tree);
                case NodeType.Or:
                    return BuildFromOr((OrTree)tree);
                case NodeType.Plus:
                    return BuildFromPlus((PlusTree)tree);
                case NodeType.Question:
                    return BuildFromQuestion((QuestionTree)tree);
                case NodeType.Star:
                    return BuildFromStar((StarTree)tree);
                case NodeType.Atom:
                    return BuildFromAtom((AtomTree)tree);
                case NodeType.Empty:
                    return BuildFromEmpty((EmptyTree)tree);
                default:
                    return null;
            }
        }

        private RegexAutomaton BuildFromConcatenation(ConcatenationTree tree)
        {
            RegexAutomaton left = BuildFromTree(tree.Left);
            RegexAutomaton right = BuildFromTree(tree.Right);

            RegexAutomaton concatenated = new RegexAutomaton();
            concatenated.AddStatesAndTransitions(left, right);

            concatenated.AddTransition(left.EndState, right.StartState);
            concatenated.StartState = left.StartState;
            concatenated.EndState = right.EndState;
            concatenated.EndState.Accepting = true;

            foreach (char atom in left.Alphabet)
            {
                concatenated.AddToAlphabet(atom);
            }
            foreach (char atom in right.Alphabet)
            {
                concatenated.AddToAlphabet(atom);
            }

            return concatenated;
        }

        private RegexAutomaton BuildFromOr(OrTree tree)
        {
            RegexAutomaton left = BuildFromTree(tree.Left);
            RegexAutomaton right = BuildFromTree(tree.Right);

            RegexAutomaton or = new RegexAutomaton();
            or.AddStatesAndTransitions(left, right);

            State start = new State();
            State end = new State();
            or.AddState(start);
            or.AddState(end);
            or.StartState = start;
            or.EndState = end;
            or.EndState.Accepting = true;

            or.AddTransition(start, left.StartState);
            or.AddTransition(start, right.StartState);
            or.AddTransition(left.EndState, end);
            or.AddTransition(right.EndState, end);

            foreach (char atom in left.Alphabet)
            {
                or.AddToAlphabet(atom);
            }
            foreach (char atom in right.Alphabet)
            {
                or.AddToAlphabet(atom);
            }

            return or;
        }

        private RegexAutomaton BuildFromQuestion(QuestionTree tree)
        {
            RegexAutomaton inner = BuildFromTree(tree.Inner);

            State start = new State();
            State end = new State(true);
            RegexAutomaton question = new RegexAutomaton();
            question.AddStatesAndTransitions(inner);
            question.AddState(start);
            question.AddState(end);
            question.AddTransition(start, inner.StartState);
            question.AddTransition(inner.EndState, end);
            question.AddTransition(start, end);
            question.StartState = start;
            question.EndState = end;
            question.EndState.Accepting = true;

            question.Alphabet = inner.Alphabet;

            return question;
        }

        private RegexAutomaton BuildFromPlus(PlusTree tree)
        {
            RegexAutomaton inner = BuildFromTree(tree.Inner);

            State start = new State();
            State end = new State(true);
            RegexAutomaton plus = new RegexAutomaton();
            plus.AddStatesAndTransitions(inner);
            plus.AddState(start);
            plus.AddState(end);
            plus.AddTransition(start, inner.StartState);
            plus.AddTransition(inner.EndState, end);
            plus.AddTransition(end, start);
            plus.StartState = start;
            plus.EndState = end;
            plus.EndState.Accepting = true;

            plus.Alphabet = inner.Alphabet;

            return plus;
        }

        private RegexAutomaton BuildFromStar(StarTree tree)
        {
            RegexAutomaton inner = BuildFromTree(tree.Inner);

            State state = new State(true);
            RegexAutomaton star = new RegexAutomaton();
            star.AddStatesAndTransitions(inner);
            star.AddState(state);
            star.AddTransition(state, inner.StartState);
            star.AddTransition(inner.EndState, state);
            star.StartState = state;
            star.EndState = state;
            star.EndState.Accepting = true;

            star.Alphabet = inner.Alphabet;

            return star;
        }

        private RegexAutomaton BuildFromAtom(AtomTree tree)
        {
            RegexAutomaton atom = new RegexAutomaton();
            State start = new State();
            State end = new State(true);
            atom.AddState(start);
            atom.AddState(end);
            atom.AddTransition(new Transition(start, end, tree.Data));
            atom.StartState = start;
            atom.EndState = end;
            atom.EndState.Accepting = true;
            atom.AddToAlphabet(tree.Data);
            return atom;
        }

        private RegexAutomaton BuildFromEmpty(EmptyTree tree)
        {
            RegexAutomaton empty = new RegexAutomaton();
            State start = new State();
            State end = new State(true);
            empty.AddState(start);
            empty.AddState(end);
            empty.AddTransition(new Transition(start, end));
            empty.StartState = start;
            empty.EndState = end;
            empty.EndState.Accepting = true;
            return empty;
        }
    }
}
