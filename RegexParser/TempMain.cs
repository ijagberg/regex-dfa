using RegexParser.Infrastructure;
using System;

namespace RegexParser
{
    class TempMain
    {
        public static void Main(string[] args)
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
