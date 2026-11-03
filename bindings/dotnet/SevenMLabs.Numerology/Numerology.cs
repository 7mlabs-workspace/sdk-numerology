using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;
namespace SevenMLabs.Numerology;
public sealed class NumerologyException : ArgumentException {
 public string Code {get;}
 public string Field {get;}
 internal NumerologyException(JsonElement error) : base(error.GetProperty("message").GetString()) {Code=error.GetProperty("code").GetString()!;Field=error.GetProperty("field").GetString()!;}
}
/// <summary>Offline, deterministic Pythagorean calculations using the bundled Rust engine.</summary>
public static class Numerology {
 public const string Version="1.0.0";
 static Numerology() {
  NativeLibrary.SetDllImportResolver(typeof(Numerology).Assembly, (name, assembly, searchPath) => {
   if(name!="sevenmlabs_numerology")return IntPtr.Zero;
   var os=OperatingSystem.IsMacOS()?"osx":OperatingSystem.IsLinux()?"linux":"unsupported";
   var arch=RuntimeInformation.ProcessArchitecture.ToString().ToLowerInvariant();
   var file=OperatingSystem.IsMacOS()?"libsevenmlabs_numerology.dylib":"libsevenmlabs_numerology.so";
   var path=Path.Combine(AppContext.BaseDirectory,"runtimes",os+"-"+arch,"native",file);
   return File.Exists(path)?NativeLibrary.Load(path):IntPtr.Zero;
  });
 }
 [DllImport("sevenmlabs_numerology",CallingConvention=CallingConvention.Cdecl)] private static extern uint numerology_abi_version();
 [DllImport("sevenmlabs_numerology",CallingConvention=CallingConvention.Cdecl)] private static extern IntPtr numerology_calculate_json_len(byte[] input, nuint length);
 [DllImport("sevenmlabs_numerology",CallingConvention=CallingConvention.Cdecl)] private static extern IntPtr numerology_version();
 [DllImport("sevenmlabs_numerology",CallingConvention=CallingConvention.Cdecl)] private static extern void numerology_free(IntPtr output);
 public static string CalculateJson(string request) {
  ArgumentNullException.ThrowIfNull(request);
  if (numerology_abi_version()!=1) throw new InvalidOperationException("Numerology ABI version differs from wrapper");
  if (Marshal.PtrToStringUTF8(numerology_version())!=Version) throw new InvalidOperationException("Numerology engine version differs from wrapper");
  var bytes=Encoding.UTF8.GetBytes(request);
  var output=numerology_calculate_json_len(bytes, (nuint)bytes.Length);
  if (output==IntPtr.Zero) throw new OutOfMemoryException("Numerology returned a null result");
  try {return Marshal.PtrToStringUTF8(output)!;} finally {numerology_free(output);}
 }
 public static JsonElement Calculate(string request) {
  using var document=JsonDocument.Parse(CalculateJson(request));
  var envelope=document.RootElement;
  if (!envelope.GetProperty("ok").GetBoolean()) throw new NumerologyException(envelope.GetProperty("error"));
  return envelope.GetProperty("result").Clone();
 }
}
