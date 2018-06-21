using System;
using System.Collections.Generic;
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

            RegexNfaConverter converter = new RegexNfaConverter();

            RegexAutomaton nfa = converter.ConvertToNfa(tree);

            DeterministicFiniteAutomaton dfa = RegexAutomaton.ConvertToDfa(nfa);

            //Debug.WriteLine($"tree: {tree}");
            //Debug.WriteLine($"nfa: {nfa}");
            //Debug.WriteLine($"dfa: {dfa}");

            return dfa;
        }

        [TestMethod]
        public void Should_Accept_Input_String_That_Is_Part_Of_Language()
        {
            Debug.WriteLine("Running test.");

            string regex = "asd*g(d)|(f)";
            string input = "asdddgd";

            DeterministicFiniteAutomaton dfa = BuildTestDfa(regex);

            Assert.AreEqual(true, dfa.MatchEntire(input));
        }

        [TestMethod]
        public void Should_Accept_Input_String_That_Is_Part_Of_Language_2()
        {
            string regex = "asd*g(d|f)";
            string input = "asgf";

            DeterministicFiniteAutomaton dfa = BuildTestDfa(regex);

            Assert.AreEqual(true, dfa.MatchEntire(input));
        }

        [TestMethod]
        public void Should_Not_Accept_Input_String_That_Is_Not_Part_Of_Language()
        {
            string regex = "s*aa(b|g)*k";
            string input = "ssaaggg";

            DeterministicFiniteAutomaton dfa = BuildTestDfa(regex);

            Assert.AreEqual(false, dfa.MatchEntire(input));
        }

        [TestMethod]
        public void Should_Accept_Correct_Number_Of_Substrings()
        {
            string regex = "(c|r)ats";
            string input = "cats and rats";

            DeterministicFiniteAutomaton dfa = BuildTestDfa(regex);
            IList<string> matchingSubstrings = dfa.MatchSubstrings(input);

            foreach(string s in matchingSubstrings)
            {
                Debug.WriteLine(s);
            }

            Assert.AreEqual(2, matchingSubstrings.Count);
        }

        [TestMethod]
        public void Should_Accept_Input_String_That_Is_Part_Of_Both_Product_Languages()
        {
            string regex1 = "a*cb";
            string regex2 = "cb";
            string input = "cb";

            DeterministicFiniteAutomaton dfa1 = BuildTestDfa(regex1);
            DeterministicFiniteAutomaton dfa2 = BuildTestDfa(regex2);

            DeterministicFiniteAutomaton product = dfa1 * dfa2;

            Debug.WriteLine($"dfa1: {dfa1}");
            Debug.WriteLine($"dfa2: {dfa2}");
            Debug.WriteLine($"product: {product}");

            Assert.AreEqual(true, product.MatchEntire(input));
        }

        [TestMethod]
        public void Should_Not_Accept_Input_String_That_Is_Not_Part_Of_Both_Product_Languages()
        {
            string regex1 = "a*cb";
            string regex2 = "cb";
            string input = "acb";

            DeterministicFiniteAutomaton dfa1 = BuildTestDfa(regex1);
            DeterministicFiniteAutomaton dfa2 = BuildTestDfa(regex2);

            DeterministicFiniteAutomaton product = dfa1 * dfa2;

            Debug.WriteLine($"dfa1: {dfa1}");
            Debug.WriteLine($"dfa2: {dfa2}");
            Debug.WriteLine($"product: {product}");

            Assert.AreEqual(false, product.MatchEntire(input));
        }
    }
}
