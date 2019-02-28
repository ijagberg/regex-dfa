// using Newtonsoft.Json;
// using Newtonsoft.Json.Converters;

// namespace RegexParser.Infrastructure
// {
//     public enum NodeType
//     {
//         Atom,
//         Star,
//         Question,
//         Plus,
//         Or,
//         Concatenation,
//         Empty
//     }

//     public class ParseTree
//     {
//         [JsonConverter(typeof(StringEnumConverter))]
//         public NodeType Type { get; set; }
//     }

//     public class OrTree : ParseTree
//     {
//         public ParseTree Left { get; }
//         public ParseTree Right { get; }

//         public OrTree(ParseTree left, ParseTree right)
//         {
//             Type = NodeType.Or;
//             Left = left;
//             Right = right;
//         }

//         public override string ToString()
//         {
//             string json = JsonConvert.SerializeObject(this, Formatting.Indented);
//             return json;
//         }
//     }

//     public class ConcatenationTree : ParseTree
//     {
//         public ParseTree Left { get; }
//         public ParseTree Right { get; }

//         public ConcatenationTree(ParseTree left, ParseTree right)
//         {
//             Type = NodeType.Concatenation;
//             Left = left;
//             Right = right;
//         }

//         public override string ToString()
//         {
//             string json = JsonConvert.SerializeObject(this, Formatting.Indented);
//             return json;
//         }
//     }

//     public class StarTree : ParseTree
//     {
//         public ParseTree Inner { get; }

//         public StarTree(ParseTree inner)
//         {
//             Type = NodeType.Star;
//             Inner = inner;
//         }

//         public override string ToString()
//         {
//             string json = JsonConvert.SerializeObject(this, Formatting.Indented);
//             return json;
//         }
//     }

//     public class QuestionTree : ParseTree
//     {
//         public ParseTree Inner { get; }

//         public QuestionTree(ParseTree inner)
//         {
//             Type = NodeType.Question;
//             Inner = inner;
//         }

//         public override string ToString()
//         {
//             string json = JsonConvert.SerializeObject(this, Formatting.Indented);
//             return json;
//         }
//     }

//     public class PlusTree : ParseTree
//     {
//         public ParseTree Inner { get; }

//         public PlusTree(ParseTree inner)
//         {
//             Type = NodeType.Plus;
//             Inner = inner;
//         }

//         public override string ToString()
//         {
//             string json = JsonConvert.SerializeObject(this, Formatting.Indented);
//             return json;
//         }
//     }

//     public class AtomTree : ParseTree
//     {
//         public char Data { get; }

//         public AtomTree(char data)
//         {
//             Type = NodeType.Atom;
//             Data = data;
//         }

//         public override string ToString()
//         {
//             string json = JsonConvert.SerializeObject(this, Formatting.Indented);
//             return json;
//         }
//     }

//     public class EmptyTree : ParseTree
//     {
//         public EmptyTree()
//         {
//             Type = NodeType.Empty;
//         }

//         public override string ToString()
//         {
//             string json = JsonConvert.SerializeObject(this, Formatting.Indented);
//             return json;
//         }
//     }
// }
