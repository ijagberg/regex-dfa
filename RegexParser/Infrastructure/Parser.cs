using RegexParser.Infrastructure;
using System.Text.RegularExpressions;


namespace RegexParser
{
    public class Parser
    {
        private string _input;
        private int _index;

        public Parser(string input)
        {
            _input = input;
            _index = 0;
        }

        private char Peek()
        {
            return _input[_index];
        }

        private void Eat(char c)
        {
            if (Peek() == c)
            {
                _index++;
            } else
            {
                // I wanted to eat something else
            }
        }

        private char Pop()
        {
            char c = Peek();
            Eat(c);
            return c;
        }

        private bool HasMore()
        {
            return (_index < _input.Length);
        }

        public ParseTree Parse()
        {
            return BuildRegEx();
        }

        private ParseTree BuildRegEx()
        {
            ParseTree termTree = BuildTerm();
            if (HasMore() && Peek() == '|')
            {
                Eat('|');
                ParseTree regexTree = BuildRegEx();
                return new OrTree(termTree, regexTree);
            } else
            {
                return termTree;
            }
        }

        private ParseTree BuildTerm()
        {
            ParseTree factorTree = new EmptyTree();
            while (HasMore() && Peek() != ')' && Peek() != '|')
            {
                ParseTree nextFactorTree = BuildFactor();
                factorTree = new ConcatenationTree(factorTree, nextFactorTree);
            }
            return factorTree;
        }

        private ParseTree BuildFactor()
        {
            ParseTree baseTree = BuildBase();
            while (HasMore() && Peek() == '*')
            {
                Eat('*');
                baseTree = new StarTree(baseTree);
            }

            return baseTree;
        }

        private ParseTree BuildBase()
        {
            switch (Peek())
            {
                case '(':
                    Eat('(');
                    ParseTree regex = BuildRegEx();
                    Eat(')');
                    return regex;
                case '\\':
                    Eat('\\');
                    char escaped = Pop();
                    return new AtomTree(escaped);
                default:
                    return new AtomTree(Pop());
            }
        }
    }
}