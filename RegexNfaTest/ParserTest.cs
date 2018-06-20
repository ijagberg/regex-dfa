using Microsoft.VisualStudio.TestTools.UnitTesting;
using RegexParser;
using RegexParser.Infrastructure;
using System;

namespace RegexNfaTest
{
    [TestClass]
    public class ParserTest
    {

        [TestMethod]
        public void SimpleRegexToTree()
        {
            string simpleRegex = "ga*";
            Parser parser = new Parser(simpleRegex);
            ParseTree result = parser.Parse();
            var output = result.ToString();
            Console.WriteLine(output);
            Console.ReadKey();
        }

    }
}
