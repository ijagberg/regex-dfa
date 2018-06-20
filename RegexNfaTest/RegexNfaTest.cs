using System;
using System.Diagnostics;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using RegexNfa;
using RegexNfa.Infrastructure;
using RegexParser;
using RegexParser.Infrastructure;

namespace RegexNfaTest
{
    [TestClass]
    public class RegexNfaTest
    {

        private DeterministicFiniteAutomaton BuildTestDfa(string regex)
        {
            Parser parser = new Parser(regex);

            ParseTree tree = parser.Parse();
            Debug.WriteLine($"tree: {tree}");

            RegexNfaConverter converter = new RegexNfaConverter();

            RegexAutomaton nfa = converter.ConvertToNfa(tree);
            Debug.WriteLine($"nfa: {nfa}");

            DeterministicFiniteAutomaton dfa = RegexAutomaton.ConvertToDfa(nfa);
            Debug.WriteLine($"dfa: {dfa}");

            return dfa;
        }

        [TestMethod]
        public void Should_Accept_Input_String_That_Is_Part_Of_Language()
        {
            Debug.WriteLine("Running test.");

            string regex = "asd*g(d)|(f)";
            string input = "asdddgd";

            DeterministicFiniteAutomaton dfa = BuildTestDfa(regex);

            Assert.AreEqual(dfa.Parse(input), true);
        }

        [TestMethod]
        public void Should_Accept_Input_String_That_Is_Part_Of_Language_2()
        {
            string regex = "asd*g(d)|(f)";
            string input = "asgf";

            DeterministicFiniteAutomaton dfa = BuildTestDfa(regex);

            Assert.AreEqual(dfa.Parse(input), true);
        }
    }
}
