using System.Text.Json.Nodes;
using SevenMLabs.Numerology;
var cases=JsonNode.Parse(File.ReadAllText(args[0]))!.AsArray();
foreach(var test in cases) {
 var actual=JsonNode.Parse(Numerology.CalculateJson(test!["request"]!.GetValue<string>()));
 if(!JsonNode.DeepEquals(actual,test["expected"])) throw new Exception("Cross-binding result differs");
}
var namesRequest="""{"operation":"names","birthDate":"2000-02-29","names":[{"id":"a","name":{"given":"Ada"}},{"id":"nick","name":{"given":"Map"}}],"combinations":[{"id":"selected","sources":[{"nameId":"a"},{"nameId":"nick"}]}]}""";
var challenges=Numerology.Calculate("""{"operation":"name","name":{"given":"Steve","family":"Jones"},"extensions":{"nameChallenges":{"nameOrder":"givenMiddleFamily"}}}""").GetProperty("data").GetProperty("extended").GetProperty("nameChallenges").GetProperty("birthName");
if(challenges.GetProperty("personality").GetProperty("difference").GetInt32()!=0 || challenges.GetProperty("personality").GetProperty("value").GetInt32()!=9 || challenges.GetProperty("soulUrge").GetProperty("value").GetInt32()!=0) throw new Exception("Published challenge convention differs");
for(var i=0;i<5000;i++) Numerology.Calculate(i%2==1?namesRequest:"""{"operation":"birth","birthDate":"2000-02-29"}""");
try {Numerology.Calculate("""{"operation":"name","name":{"given":"123"}}""");throw new Exception("Error expected");}
catch(NumerologyException e) {if(e.Code!="UNSUPPORTED_CHARACTER")throw;}
Console.WriteLine($"{{\"runtime\":\"{Environment.Version}\",\"cases\":{cases.Count},\"repeatedCalculations\":5000,\"repeatedNamesCalculations\":2500,\"status\":\"passed\"}}");
