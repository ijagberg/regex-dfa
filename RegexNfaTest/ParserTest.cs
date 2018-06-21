using Microsoft.VisualStudio.TestTools.UnitTesting;
using RegexParser;
using RegexParser.Infrastructure;
using System;
using System.Diagnostics;

namespace RegexNfaTest
{
    [TestClass]
    public class ParserTest
    {

        [TestMethod]
        public void Should_Output_Correct_ParseTree()
        {
            string simpleRegex = "(asd*gd)|(f)";

            Parser parser = new Parser(simpleRegex);
            ParseTree result = parser.Parse();

            Debug.WriteLine(result);
        }
    }
}
