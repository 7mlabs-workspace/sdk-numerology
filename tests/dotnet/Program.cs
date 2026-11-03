using System.Text.Json.Nodes;
using SevenMLabs.Numerology;
var cases=JsonNode.Parse(File.ReadAllText(args[0]))!.AsArray();
foreach(var test in cases) {
 var actual=JsonNode.Parse(Numerology.CalculateJson(test!["request"]!.GetValue<string>()));
 if(!JsonNode.DeepEquals(actual,test["expected"])) throw new Exception("Cross-binding result differs");
}
for(var i=0;i<5000;i++) Numerology.Calculate("""{"operation":"birth","birthDate":"2000-02-29"}""");
try {Numerology.Calculate("""{"operation":"name","name":{"given":"123"}}""");throw new Exception("Error expected");}
catch(NumerologyException e) {if(e.Code!="UNSUPPORTED_CHARACTER")throw;}
Console.WriteLine($"{{\"runtime\":\"{Environment.Version}\",\"cases\":{cases.Count},\"repeatedCalculations\":5000,\"status\":\"passed\"}}");
